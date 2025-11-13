use std::error::Error;
use teloxide::{
    prelude::*,
    sugar::request,
    types::{
        FileId, InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResult,
        InlineQueryResultArticle, InlineQueryResultCachedGif, InlineQueryResultGif,
        InlineQueryResultsButton, InputMessageContent, InputMessageContentText, Me,
    },
    utils::command::BotCommands,
};
use url::Url;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are available:"
)]
enum Command {
    #[command(description = "Display help message.")]
    Help,
    #[command(description = "Welcome message.")]
    Start,
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let debian_versions = [
        "Buzz", "Rex", "Bo", "Hamm", "Slink", "Potato", "Woody", "Sarge", "Etch", "Lenny",
        "Squeeze", "Wheezy", "Jessie", "Stretch", "Buster", "Bullseye",
    ];

    for versions in debian_versions.chunks(3) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

#[tracing::instrument]
async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let username = msg
        .from
        .as_ref()
        .and_then(|user| user.username.as_deref())
        .unwrap_or("unknown");
    let user_id = msg.from.as_ref().map(|user| user.id).unwrap_or(UserId(0));

    match msg.text() {
        Some(text) => {
            tracing::info!(
                "Text message from @{} (user_id: {}): {}",
                username,
                user_id,
                text
            );
            match BotCommands::parse(text, me.username()) {
                Ok(Command::Help) => {
                    let command_descriptions = Command::descriptions().to_string();
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Use this bot inline by typing @HowOrcBot in any chat.\n\n{}",
                            command_descriptions
                        ),
                    )
                    .await?;
                }
                Ok(Command::Start) => {
                    bot.send_message(
                        msg.chat.id,
                        concat!(
                            "This is some random bot made just for fun.\n",
                            "Use it inline by typing @HowOrcBot in any chat.\n",
                            "Type /help to see available commands."
                        )
                        .to_string(),
                    )
                    .await?;
                }
                Err(_) => {
                    bot.send_message(msg.chat.id, "Command not found!").await?;
                }
            }
        }
        None => {
            tracing::info!(
                "Received non-text message from @{} (user_id: {})",
                username,
                user_id
            );
        }
    }

    Ok(())
}

#[tracing::instrument]
async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let orc_gif_id =
    //     "CgACAgQAAxkBAAEEmP5pFgF6tVWz5Lk0Q_e083b1_EbZGgACcwQAAkYs9VFt6Rdx0-6eJjYE".to_string();
    // let orc_gif = InlineQueryResultCachedGif::new("1", FileId(orc_gif_id));

    let orc_gif = InlineQueryResultGif::new(
        "2",
        Url::parse("https://i.ibb.co/6RpRZm63/orc-smile.gif")?,
        Url::parse("https://i.ibb.co/6RpRZm63/orc-smile.gif")?,
    )
    .title("Title")
    .caption("Caption");

    let start_button = InlineQueryResultArticle::new(
        "1",
        "How much of an orc are you?",
        InputMessageContent::Text(InputMessageContentText::new(
            "Hello! I'm HowOrcBot. Use me inline by typing @HowOrcBot in any chat!",
        )),
    )
    .description("😈 Find out how much of an orc you are!")
    .thumbnail_url(Url::parse("https://i.ibb.co/6RpRZm63/orc-smile.gif")?);

    bot.answer_inline_query(
        q.id,
        vec![
            InlineQueryResult::Article(start_button),
            InlineQueryResult::Gif(orc_gif),
        ],
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
