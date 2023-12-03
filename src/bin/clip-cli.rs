use clipshare::domain::clip::field::{Content, ExpiresAt, Password, ShortCode, Title};
use clipshare::service::ask::{GetClip, NewClip, UpdateClip};
use clipshare::web::api::{ApiKey, API_KEY_HEADER};
use clipshare::Clip;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        short_code: ShortCode,

        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "content")]
        clip: String,

        #[structopt(short, long, help = "password")]
        password: Option<Password>,

        #[structopt(short, long, help = "expiration date")]
        expires_at: Option<ExpiresAt>,

        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
    Update {
        short_code: ShortCode,
        clip: String,

        #[structopt(short, long, help = "password")]
        password: Option<Password>,

        #[structopt(short, long, help = "expiration date")]
        expires_at: Option<ExpiresAt>,

        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clip-cli", about = "Clip Share API Cli Tool")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSHARE_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn get_clip(addr: &str, ask_svc: GetClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip/{}", addr, ask_svc.short_code.into_inner());
    let mut request = client.get(addr);
    request = match ask_svc.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE, format!("password={}", password)),
        None => request,
    };

    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_clip(addr: &str, ask_svc: NewClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.post(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn update_clip(addr: &str, ask_svc: UpdateClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.put(addr);

    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            short_code,
            password,
        } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                short_code,
            };
            let clip = get_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::New {
            clip,
            password,
            expires_at,
            title,
        } => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                exprires_at: expires_at.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            let clip = new_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::Update {
            clip,
            password,
            expires_at,
            title,
            short_code,
        } => {
            let password = password.unwrap_or_default();
            let svc_req = GetClip {
                password: password.clone(),
                short_code: short_code.clone(),
            };
            let original_clip = get_clip(opt.addr.as_str(), svc_req, opt.api_key.clone())?;
            let svc_req = UpdateClip {
                content: Content::new(clip.as_str())?,
                exprires_at: expires_at.unwrap_or(original_clip.expires_at),
                title: title.unwrap_or(original_clip.title),
                password,
                short_code,
            };

            let clip = update_clip(opt.addr.as_str(), svc_req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}
