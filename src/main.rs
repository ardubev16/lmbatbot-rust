use rand::seq::SliceRandom;
use std::env;
use teloxide::{
    dispatching::UpdateHandler, macros::BotCommands, prelude::*,
    types::InputFile, RequestError,
};
mod db;
mod md_escape;
mod tag_group;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting lmbatbot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        // .dependencies(dptree::deps![parameters])
        // If no handler succeeded to handle an update, this closure will be called.
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler.
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Fun commands:")]
enum FunCommands {
    #[command(
        description = "Sends a random sticker from a given sticker pack"
    )]
    Bocchi,
}

async fn bocchi(bot: Bot, msg: Message) -> ResponseResult<()> {
    let sticker_set_name =
        env::var("STICKER_SET_NAME").expect("STICKER_SET_NAME not set");
    let sticker_set = bot.get_sticker_set(sticker_set_name).await?;
    let sticker = sticker_set
        .stickers
        .choose(&mut rand::thread_rng())
        .expect("No stickers in set");

    let sticker_file = InputFile::file_id(sticker.file.id.clone());

    bot.send_sticker(msg.chat.id, sticker_file).await?;

    Ok(())
}

fn schema() -> UpdateHandler<RequestError> {
    dptree::entry().branch(tag_group::handler()).branch(
        Update::filter_message()
            .filter_command::<FunCommands>()
            .branch(dptree::case![FunCommands::Bocchi].endpoint(bocchi)),
    )
}
