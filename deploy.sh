#!/usr/bin/env bash

set -e

usage() {
    cat << EOF
Usage: $0 [-ud]
    -u: Builds and brings up the bot
    -d: Brings down the bot
EOF
}

if [ $# -ne 1 ]; then
    usage
    exit 1
fi

if [ "$1" = "-u" ]; then
    docker compose down
    git pull
    docker compose up --build -d
elif [ "$1" = "-d" ]; then
    docker compose down
else
    usage
    exit 1
fi
