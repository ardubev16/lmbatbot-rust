use rand::seq::SliceRandom;
use std::env;
use teloxide::{
    dispatching::UpdateHandler, macros::BotCommands, prelude::*,
    requests::ResponseResult, types::InputFile, RequestError,
};

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

pub fn handler() -> UpdateHandler<RequestError> {
    Update::filter_message()
        .filter_command::<FunCommands>()
        .branch(dptree::case![FunCommands::Bocchi].endpoint(bocchi))
}
