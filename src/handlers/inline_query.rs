use rand::Rng;
use std::error::Error;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
};
use url::Url;

const THUMBNAIL_SIZE: u32 = 80;
const ORC_QUERY_ID: &str = "1";
const HELP_QUERY_ID: &str = "2";

#[tracing::instrument]
pub async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
    assets_url: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Inline query from @{} (user_id: {}): {}",
        q.from.username.as_deref().unwrap_or("unknown"),
        q.from.id,
        q.query
    );

    let orc_percentage = {
        let mut rng = rand::rng();
        rng.random_range(0..=100)
    };

    let orc_button = InlineQueryResultArticle::new(
        ORC_QUERY_ID,
        "😈 How much of an orc are you?",
        InputMessageContent::Text(InputMessageContentText::new(&format!(
            "You're {}% orc! 😈",
            orc_percentage
        ))),
    )
    .description("Find out how much of an orc you are!")
    .thumbnail_url(Url::parse(&format!("{}orc-smile-80x80.jpg", assets_url))?)
    .thumbnail_width(THUMBNAIL_SIZE)
    .thumbnail_height(THUMBNAIL_SIZE);

    let help_button = InlineQueryResultArticle::new(
        HELP_QUERY_ID,
        "❔ Need some help?",
        InputMessageContent::Text(InputMessageContentText::new(
            "Hello! I'm HowOrcBot. Just type @HowOrcBot in any chat to use me inline!",
        )),
    )
    .description("Let's see how I can help you!")
    .thumbnail_url(Url::parse(&format!("{}orc-80x80.jpg", assets_url))?)
    .thumbnail_width(THUMBNAIL_SIZE)
    .thumbnail_height(THUMBNAIL_SIZE);

    bot.answer_inline_query(
        q.id,
        vec![
            InlineQueryResult::Article(orc_button),
            InlineQueryResult::Article(help_button),
        ],
    )
    .await?;

    Ok(())
}
