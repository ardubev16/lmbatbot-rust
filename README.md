# LMBATBOT

## Deployment

Set the following environment variables in the `.env` file:

- **TELEGRAM_TOKEN**: your bot's secret token
- **MONGO_USERNAME**: the username for the local DB
- **MONGO_PASSOWRD**: the password for the local DB

Needs `docker compose` installed, to deploy run `./deploy.sh -u`
