use crate::{
    db, md_escape,
    types::{HandlerError, HandlerResult},
    utils::{parse_tagadd_args, send_usage},
};
use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{MessageEntityKind, ParseMode},
    utils::{command::BotCommands, markdown},
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
    #[command(description = "Adds a tag group")]
    TagAdd { args: String },
    #[command(description = "Deletes a tag group")]
    TagDelete { group: String },
}

async fn taglist(bot: Bot, msg: Message) -> HandlerResult {
    let collection = get_collection();

    let mut tags = collection
        .find(doc! { "chat_id": msg.chat.id.0 }, None)
        .await?;

    let mut text = vec![markdown::bold("Groups:")];
    while tags.advance().await? {
        let tag = tags.deserialize_current()?;
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
        .disable_notification(true)
        .await?;

    Ok(())
}

async fn tagadd(
    bot: Bot,
    msg: Message,
    group: String,
    emoji: String,
    names: Vec<String>,
) -> HandlerResult {
    let collection = get_collection();

    let group = format!("#{}", group);
    let names = names
        .iter()
        .map(|s| s.trim_start_matches('@').to_owned())
        .collect::<Vec<_>>();

    let res = collection
        .update_one(
            doc! { "chat_id": msg.chat.id.0, "group": &group},
            doc! {
                "$set": {"emoji": emoji, "names": names},
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await?;

    let text = match res.matched_count {
        0 => format!("Added group {}.", group),
        _ => format!("Updated group {}.", group),
    };

    bot.send_message(msg.chat.id, text)
        .disable_notification(true)
        .await?;

    Ok(())
}

async fn tagdelete(bot: Bot, msg: Message, group: String) -> HandlerResult {
    let collection = get_collection();

    let group = format!("#{}", group);
    let res = collection
        .delete_one(
            doc! {
                "chat_id": msg.chat.id.0,
                "group": &group
            },
            None,
        )
        .await?;

    let text = match res.deleted_count {
        1 => format!("Deleted group {}.", group),
        _ => format!("WARNING: Group {} not found.", group),
    };

    bot.send_message(msg.chat.id, text)
        .disable_notification(true)
        .await?;

    Ok(())
}

async fn tag_handler(bot: Bot, msg: Message) -> HandlerResult {
    let collection = get_collection();

    // NOTE: tags should not be empty because this handler is only called if there are tags
    let tags = msg
        .parse_entities()
        .expect("Failed to parse entities")
        .iter()
        .filter(|entity| matches!(entity.kind(), MessageEntityKind::Hashtag))
        .map(|entity| entity.text().to_string())
        .collect::<Vec<_>>();

    let mut found_groups = collection
        .find(
            doc! {
                "chat_id": msg.chat.id.0,
                "group": { "$in": &tags },
            },
            None,
        )
        .await?;

    let mut emojis = Vec::new();
    let mut tag_list = HashSet::new();
    while found_groups.advance().await? {
        let group = found_groups.deserialize_current()?;
        emojis.push(group.emoji);
        group.names.into_iter().for_each(|name| {
            tag_list.insert(name);
        });
    }
    if tag_list.is_empty() {
        return Err(HandlerError::Custom("No tags found.".into()));
    }

    let sender = msg.from().unwrap().username.as_ref().expect("No username");
    let tag_list = tag_list
        .into_iter()
        .filter(|tag| sender.ne(tag))
        .map(|tag| format!("@{}", tag))
        .collect::<Vec<_>>()
        .join(" ");

    let text = md_escape::italic(
        format!("{} {}\n\n{}", emojis.join(""), tags.join(" "), tag_list)
            .as_str(),
    );

    // TODO: check if the tagged users get notified with "disable_notification: true"
    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}

pub fn handler() -> UpdateHandler<HandlerError> {
    use dptree::case;

    let command_handler = dptree::entry()
        .filter_command::<GroupCommands>()
        .branch(case![GroupCommands::TagList].endpoint(taglist))
        .branch(case![GroupCommands::TagAdd { args }].endpoint(
            |bot, msg, args| async move {
                match parse_tagadd_args(args) {
                    Ok((group, emoji, names)) => {
                        tagadd(bot, msg, group, emoji, names).await
                    }
                    Err(_) => {
                        send_usage(
                            &bot,
                            msg.chat.id,
                            "/tagadd <name>\n\
                             <emoji>\n\
                             <@tag1> <@tag2> ...",
                        )
                        .await?;
                        Ok(())
                    }
                }
            },
        ))
        .branch(case![GroupCommands::TagDelete { group }].endpoint(
            |bot, msg: Message, group: String| async move {
                if group.is_empty() {
                    send_usage(&bot, msg.chat.id, "/tagdelete <group>").await?;
                    Ok(())
                } else {
                    tagdelete(bot, msg, group).await
                }
            },
        ));

    Update::filter_message()
        .filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
        .branch(command_handler)
        .branch(
            dptree::filter(|msg: Message| {
                msg.parse_entities().unwrap().iter().any(|entity| {
                    matches!(entity.kind(), &MessageEntityKind::Hashtag)
                })
            })
            .endpoint(tag_handler),
        )
}
