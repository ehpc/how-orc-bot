use teloxide::{prelude::*, update_listeners::webhooks};
mod handlers;
use handlers::{inline_query_handler, message_handler};
mod error_handlers;
use error_handlers::TracingErrorHandler;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let bot = Bot::from_env();
    let assets_url = std::env::var("ASSETS_URL").expect("ASSETS_URL must be set");

    let port: u16 = std::env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");
    let addr = ([127, 0, 0, 1], port).into();
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL env variable is not set");
    let url = webhook_url.parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![assets_url])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            TracingErrorHandler::with_custom_text("Webhook error"),
        )
        .await;
}
