use commands::{count_words, fun, tag_group};
use teloxide::{dispatching::UpdateHandler, prelude::*, types::BotCommand};
use types::CommandError;

mod commands;
mod db;
mod md_escape;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting lmbatbot...");

    let bot = Bot::from_env();
    bot.set_my_commands(command_list())
        .await
        .expect("Failed to set commands");

    Dispatcher::builder(bot, schema())
        // Here you specify initial dependencies that all handlers will receive; they can be
        // database connections, configurations, and other auxiliary arguments. It is similar to
        // `actix_web::Extensions`.
        // .dependencies(dptree::deps![parameters])
        // .default_handler(|upd| async move {
        //     log::warn!("Unhandled update: {:?}", upd);
        // })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn command_list() -> Vec<BotCommand> {
    let mut commands = Vec::new();
    commands.extend(tag_group::commands());
    commands.extend(fun::commands());
    commands.extend(count_words::commands());
    commands
}

fn schema() -> UpdateHandler<CommandError> {
    dptree::entry()
        // FIXME: Find a way to make a handler always run
        // .branch(Update::filter_message().endpoint(count_words::tracker))
        .branch(tag_group::handler())
        .branch(fun::handler())
        .branch(count_words::handler())
}
