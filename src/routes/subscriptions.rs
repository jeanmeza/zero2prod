use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name = %_form.name
    );
    // Using `enter` in an async function is a recipe for disaster!
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        insert into subscriptions (id, email, name, subscribed_at)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool`
    //  wrapped by `web::Data`.
    .execute(_pool.get_ref())
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // Yes, this error log falls outside of `query_span`
            // We'll rectify it later, pinky swear!
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
