use super::model;
use crate::{
    data::{DataError, DatabasePool},
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
