use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(_form, _pool),
    fields(
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    )
)]
pub async fn subscribe(_form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match _form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&new_subscriber, &_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, _pool)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    _pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into subscriptions (id, email, name, subscribed_at, status)
        values ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool`
    //  wrapped by `web::Data`.
    .execute(_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early if the function failed, returning sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
