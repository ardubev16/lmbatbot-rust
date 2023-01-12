use teloxide::{dispatching::UpdateHandler, prelude::*, RequestError};
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

fn schema() -> UpdateHandler<RequestError> {
    dptree::entry().branch(tag_group::handler())
}
