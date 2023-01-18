# LMBATBOT

"Lonigo Maggiore di Brenta Arzignano Trento Bot"

A general purpose Telegram Bot with some useful and some funny features.

## Features

### Group Tags

This feature allows you to create sub-groups of people which are inside the same group so you are able to use a single tag to refer to multiple people. Each group also has an emoji to quickly see which group has been tagged.

- To create or update a group use the command `/tagadd` with the following format:

```
/tagadd <name>
<emoji>
<@tag1> <@tag2> ...
```

- To delete a group use the command `/tagdelete <name>`
- To list all available groups use the command `/taglist`
- To tag all the components of one or more groups use one or more `#name` inside the message. The bot will reply with a message containing all the tags of those groups without yours

### Fun Commands

Use the command `/bocchi` to make the bot send a random sticker from the sticker-pack which is set in the environment variable `STICKER_SET_NAME`

## Deployment

Set the following environment variables in the `.env` file:

- **TELEGRAM_TOKEN**: your bot's secret token
- **MONGO_USERNAME**: the username for the local DB
- **MONGO_PASSOWRD**: the password for the local DB
- **STICKER_SET_NAME**: the name of the sticker-pack used in the `/bocchi` command

Needs `docker compose` installed, to deploy run `./deploy.sh -u`
