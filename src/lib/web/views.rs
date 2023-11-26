use crate::data::DatabasePool;
use crate::service::{self, ServiceError};
use crate::ShortCode;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

type ViewStore = Arc<Mutex<HashMap<ShortCode, u32>>>;

#[derive(Debug, thiserror::Error)]
enum ViewsError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),

    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<ViewMsg>),
}

enum ViewMsg {
    Commit,
    View(ShortCode, u32),
}

pub struct Views {
    tx: Sender<ViewMsg>,
}

impl Views {
    fn commit_views(
        store: ViewStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), ViewsError> {
        let store = Arc::clone(&store);

        let store: Vec<(ShortCode, u32)> = {
            let mut store = store.lock();
            let store_vec = store.iter().map(|(k, v)| (k.clone(), *v)).collect();
            store.clear();
            store_vec
        };

        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;
            for (short_code, views) in store {
                if let Err(e) = service::action::increase_views(&short_code, views, &pool).await {
                    eprintln!("error increasing views: {}", e);
                }
            }
            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    fn process_msg(
        msg: ViewMsg,
        store: ViewStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), ViewsError> {
        match msg {
            ViewMsg::Commit => Self::commit_views(store.clone(), handle.clone(), pool.clone())?,
            ViewMsg::View(short_code, views) => {
                let mut view_count = store.lock();
                let view_count = view_count.entry(short_code).or_insert(0);
                *view_count += views;
            }
        }

        Ok(())
    }

    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("Views thread spawned");
            let store: ViewStore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::process_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("message processing error: {}", e);
                        }
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = tx_clone.send(ViewMsg::Commit) {
                                eprintln!("error sending commit msg to views channel: {}", e);
                            }
                        }
                        _ => break,
                    },
                }
            }
        });
        Self { tx }
    }

    pub fn view(&self, short_code: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(ViewMsg::View(short_code, count)) {
            eprintln!("view count error: {}", e);
        }
    }
}
