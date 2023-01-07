#!/usr/bin/env bash

while true; do 
    docker compose up --build 
    docker compose down 
    echo -n "Quit? [y/N] "
    read -r quit
    [[ $quit == "y" ]] && exit 0
done
