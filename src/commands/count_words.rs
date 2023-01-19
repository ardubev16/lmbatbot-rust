use crate::{
    db, md_escape,
    types::{HandlerError, HandlerResult},
    utils::send_usage,
};
use mongodb::{
    bson::doc,
    options::{FindOptions, UpdateOptions},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{BotCommand, ParseMode},
    utils::{command::BotCommands, markdown},
};

#[derive(Debug, Serialize, Deserialize)]
struct WordCnt {
    chat_id: i64,
    word: String,
    count: u32,
}

fn get_collection() -> mongodb::Collection<WordCnt> {
    db::get_db().collection("word_cnt")
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Word Counter commands:")]
enum WcCommands {
    #[command(
        description = "Show how many times tracked words have been said"
    )]
    Stats,
    #[command(description = "Add a word to the tracked list")]
    WordAdd { word: String },
    #[command(description = "Remove a word from the tracked list")]
    WordDelete { word: String },
}

async fn stats(bot: Bot, msg: Message) -> HandlerResult<()> {
    let collection = get_collection();

    log::debug!("Getting stats for chat {}", msg.chat.id);
    let mut words = collection
        .find(
            doc! { "chat_id": msg.chat.id.0 },
            FindOptions::builder().sort(doc! { "count": -1 }).build(),
        )
        .await?;

    let mut text = vec![markdown::bold("Stats:\n")];
    while words.advance().await? {
        let word = words.deserialize_current()?;
        text.push(markdown::escape(
            format!("{}: {}", word.word, word.count).as_str(),
        ));
    }

    bot.send_message(msg.chat.id, text.join("\n"))
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .await?;

    Ok(())
}

async fn wordadd(bot: Bot, msg: Message, word: String) -> HandlerResult<()> {
    let collection = get_collection();

    let word = word.to_lowercase();
    let res = collection
        .update_one(
            doc! { "chat_id": msg.chat.id.0, "word": &word},
            doc! { "$set": {"count": 0}, },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    let word = md_escape::bold(&word);
    let text = match res.matched_count {
        0 => format!("Added word {}", word),
        _ => format!("WARNING: Counter for {} has been reset", word),
    };

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .await?;

    Ok(())
}

async fn worddelete(bot: Bot, msg: Message, word: String) -> HandlerResult<()> {
    let collection = get_collection();

    let word = word.to_lowercase();
    let res = collection
        .delete_one(
            doc! {
                "chat_id": msg.chat.id.0,
                "word": &word
            },
            None,
        )
        .await?;

    let word = md_escape::bold(&word);
    let text = match res.deleted_count {
        1 => format!("Deleted word {}", word),
        _ => format!("WARNING: Word {} not found", word),
    };

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .await?;

    Ok(())
}

pub async fn tracker(_: Bot, msg: Message) -> HandlerResult<()> {
    let collection = get_collection();

    let mut words = collection
        .find(
            doc! {
                "chat_id": msg.chat.id.0
            },
            None,
        )
        .await?;

    log::debug!("tracked message: {:?}", msg);
    while words.advance().await? {
        let word = words.deserialize_current()?;
        let re = Regex::new(&format!(r"\b{}\b", word.word)).unwrap();
        let count = re.find_iter(&msg.text().unwrap()).count();
        if count > 0 {
            collection
                .update_one(
                    doc! {
                        "chat_id": msg.chat.id.0,
                        "word": word.word,
                    },
                    doc! {
                        "$inc": {"count": count as u32}
                    },
                    None,
                )
                .await?;
        }
    }

    Ok(())
}

pub fn handler() -> UpdateHandler<HandlerError> {
    use dptree::case;

    Update::filter_message()
        .filter_command::<WcCommands>()
        .branch(case![WcCommands::Stats].endpoint(stats))
        .branch(case![WcCommands::WordAdd { word }].endpoint(
            |bot, msg: Message, word: String| async move {
                if word.is_empty() {
                    send_usage(&bot, msg.chat.id, "/wordadd <word>").await?;
                    Ok(())
                } else {
                    wordadd(bot, msg, word).await
                }
            },
        ))
        .branch(case![WcCommands::WordDelete { word }].endpoint(
            |bot, msg: Message, word: String| async move {
                if word.is_empty() {
                    send_usage(&bot, msg.chat.id, "/worddelete <word>").await?;
                    Ok(())
                } else {
                    worddelete(bot, msg, word).await
                }
            },
        ))
}

pub fn commands() -> Vec<BotCommand> {
    WcCommands::bot_commands()
}
