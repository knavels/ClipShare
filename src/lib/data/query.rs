use super::model;
use crate::{
    data::{DataError, DatabasePool},
    web::api::ApiKey,
    ShortCode,
};

type Result<T> = std::result::Result<T, DataError>;

pub async fn increase_views(short_code: &ShortCode, views: u32, pool: &DatabasePool) -> Result<()> {
    let short_code = short_code.as_str();
    Ok(sqlx::query!(
        "UPDATE clips SET views = views + ? WHERE short_code = ?",
        views,
        short_code
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let short_code = model.short_code.as_str();

    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE short_code = ?",
        short_code
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();

    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            id, short_code, content, title, created_at, expires_at, password, views
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.id,
        model.short_code,
        model.content,
        model.title,
        model.created_at,
        model.expires_at,
        model.password,
        0
    )
    .execute(pool)
    .await?;

    get_clip(model.short_code, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
            content = ?,
            expires_at = ?,
            password = ?,
            title = ?
        WHERE short_code = ?"#,
        model.content,
        model.expires_at,
        model.password,
        model.title,
        model.short_code,
    )
    .execute(pool)
    .await?;

    get_clip(model.short_code, pool).await
}

pub async fn generate_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();
    sqlx::query!("INSERT INTO api_keys (api_key) VALUES (?)", bytes)
        .execute(pool)
        .await
        .map(|_| ())?;
    Ok(api_key)
}

pub enum RevocationStatus {
    Revoked,
    NotFound,
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key == ?", bytes)
            .execute(pool)
            .await
            .map(|result| match result.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked,
            })?,
    )
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    use sqlx::Row;
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys WHERE api_key = ?")
            .bind(bytes)
            .fetch_one(pool)
            .await
            .map(|row| {
                let count: u32 = row.get(0);
                count > 0
            })?,
    )
}

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
        sqlx::query!(r#"DELETE FROM clips WHERE strftime('%s', 'now') > expires_at"#)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

#[cfg(test)]
pub mod test {
    use crate::data::test::*;
    use crate::data::*;
    use crate::test::async_runtime;

    fn model_get_clip(short_code: &str) -> model::GetClip {
        model::GetClip {
            short_code: short_code.into(),
        }
    }

    fn model_new_clip(short_code: &str) -> model::NewClip {
        use chrono::Utc;
        model::NewClip {
            id: DbId::new().into(),
            content: format!("content for clip '{}'", short_code),
            title: None,
            short_code: short_code.into(),
            created_at: Utc::now().timestamp(),
            expires_at: None,
            password: None,
        }
    }

    #[test]
    fn clip_new_and_get() {
        let rt = async_runtime();
        let db = new_db(rt.handle());
        let pool = db.get_pool();

        let clip =
            rt.block_on(async move { super::new_clip(model_new_clip("1"), &pool.clone()).await });

        assert!(clip.is_ok());

        let clip = clip.unwrap();

        assert!(clip.short_code == "1");
        assert!(clip.content == "content for clip '1'");
    }
}
