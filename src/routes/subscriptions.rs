use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use sqlx::PgPool;

use crate::domain::{
    new_subscriber::NewSubscriber, subscriber_email::SubscriberEmail,
    subscriber_name::SubscriberName,
};

fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
    Ok(NewSubscriber {
        name: SubscriberName::parse(form.name.clone())?,
        email: SubscriberEmail::parse(form.email.clone())?,
    })
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(name="Adding a new subscriber", skip(form, pool), fields(subscriber_email = %form.email, subscriber_name = %form.name))]
pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match parse_subscriber(form.0) {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved!");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1,$2,$3,$4)"#,
        uuid::Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
