use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting lmbatbot...");

    let bot = Bot::from_env();

    Command::repl(bot, handler).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Tag group commands:")]
enum Command {
    #[command(description = "Display this message")]
    Help,
    #[command(description = "Lists available tags")]
    TagList,
    #[command(
        description = "Adds a tag group",
        // FIXME: newline separator doesn't seem to work, probably will have to use a custom parser
        parse_with = "split",
        separator = "\n"
    )]
    TagAdd(String, String, String),
    #[command(description = "Deletes a tag group")]
    TagDelete(String),
}

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let answer = match cmd {
        Command::Help => Command::descriptions().to_string(),
        Command::TagList => "tag list".to_string(),
        Command::TagAdd(tag, text, description) => {
            format!("tag add: {} {} {}", tag, text, description)
        }
        Command::TagDelete(tag) => format!("tag delete: {}", tag),
    };

    bot.send_message(msg.chat.id, answer).await?;

    Ok(())
}
