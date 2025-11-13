use std::error::Error;
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};

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

#[tracing::instrument]
pub async fn message_handler(
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
                            "Just type @HowOrcBot in any chat to use this bot inline.\n\n{}",
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
                            "Just type @HowOrcBot in any chat to use it inline.\n",
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
