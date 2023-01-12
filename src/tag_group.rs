use crate::md_escape;
use crate::{db, utils};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{MessageEntityKind, ParseMode},
    utils::{command::BotCommands, markdown},
    RequestError,
};

// TODO: use user_id instead of username, then use "markdown::user_mention"
#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    chat_id: i64,
    group: String,
    emoji: String,
    names: Vec<String>,
}

fn get_collection() -> mongodb::Collection<Tag> {
    db::get_db().collection("tags")
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Tag group commands:")]
enum GroupCommands {
    #[command(description = "Lists available tags")]
    TagList,
    #[command(
        description = "Adds a tag group",
        parse_with = utils::parse_3_nl_args,
    )]
    TagAdd {
        name: String,
        emoji: String,
        tags: String,
    },
    #[command(description = "Deletes a tag group")]
    TagDelete { name: String },
}

async fn taglist(bot: Bot, msg: Message) -> ResponseResult<()> {
    let collection = get_collection();

    let mut tags = collection
        .find(doc! { "chat_id": msg.chat.id.0 }, None)
        .await
        .expect("Failed to execute find.");

    let mut text = vec![markdown::bold("Groups:")];
    while tags.advance().await.expect("Failed to get next tag.") {
        let tag = tags.deserialize_current().unwrap();
        let names = tag.names.join(", ");
        text.push(markdown::escape(
            format!("{} {}: {}", tag.emoji, tag.group, names).as_str(),
        ));
    }

    let text = match text.len() {
        1 => markdown::bold("No groups found."),
        _ => text.join("\n\n"),
    };

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

async fn tagadd(
    bot: Bot,
    msg: Message,
    name: String,
    emoji: String,
    tags: String,
) -> ResponseResult<()> {
    // TODO: tagadd
    bot.send_message(msg.chat.id, "tagadd Not implemented yet")
        .await?;

    Ok(())
}

async fn tagdelete(bot: Bot, msg: Message, name: String) -> ResponseResult<()> {
    // TODO: tagdelete
    bot.send_message(msg.chat.id, "tagdelete Not implemented yet")
        .await?;

    Ok(())
}

async fn tag_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let collection = get_collection();

    // NOTE: tags should not be empty because this handler is only called if there are tags
    let tags = msg
        .parse_entities()
        .expect("Failed to parse entities")
        .iter()
        .filter(|entity| matches!(entity.kind(), MessageEntityKind::Hashtag))
        .map(|entity| entity.text())
        .collect::<Vec<_>>();

    let mut found_groups = collection
        .find(
            doc! {
                "chat_id": msg.chat.id.0,
                "group": { "$in": tags },
            },
            None,
        )
        .await
        .unwrap();

    let mut emojis = Vec::new();
    let mut tag_list = HashSet::new();
    while found_groups
        .advance()
        .await
        .expect("Failed to get next group.")
    {
        let group = found_groups.deserialize_current().unwrap();
        emojis.push(group.emoji);
        group.names.into_iter().for_each(|name| {
            tag_list.insert(name);
        });
    }
    if tag_list.is_empty() {
        return Ok(());
    }

    let sender = msg.from().unwrap().username.as_ref().expect("No username");
    let tag_list = tag_list
        .into_iter()
        .filter(|tag| sender.ne(tag))
        .map(|tag| format!("@{}", tag))
        .collect::<Vec<_>>()
        .join(" ");

    // FIXME: find a way to get message text formatted with markdown
    let content = msg.text().unwrap();

    let text = format!(
        "{} @{}\n\
         {}\n\
         \n\
         {}",
        emojis.join(""),
        md_escape::italic(sender.as_str()),
        // FIXME: remove this after fixing the previous FIXM
        markdown::escape(content),
        md_escape::italic(tag_list.as_str()),
    );

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: GroupCommands,
) -> ResponseResult<()> {
    match cmd {
        GroupCommands::TagList => taglist(bot, msg).await,
        GroupCommands::TagAdd { name, emoji, tags } => {
            tagadd(bot, msg, name, emoji, tags).await
        }
        GroupCommands::TagDelete { name } => tagdelete(bot, msg, name).await,
    }
}

pub fn handler() -> UpdateHandler<RequestError> {
    Update::filter_message()
        .filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
        .branch(
            dptree::entry()
                .filter_command::<GroupCommands>()
                .endpoint(command_handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.parse_entities().unwrap().iter().any(|entity| {
                    matches!(entity.kind(), &MessageEntityKind::Hashtag)
                })
            })
            .endpoint(tag_handler),
        )
}
