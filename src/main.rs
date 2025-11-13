use teloxide::prelude::*;
mod handlers;
use handlers::{inline_query_handler, message_handler};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let bot = Bot::from_env();
    let assets_url = std::env::var("ASSETS_URL").expect("ASSETS_URL must be set");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![assets_url])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
