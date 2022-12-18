#!/bin/bash

if tmux list-windows | grep -q 'ambrosioBot' 
then
    echo "Session exists"
else
    echo "Session does not exist, creating sesion";
    tmux new-session -s ambrosioBot;
fi

tmux send-keys -t ambrosioBot "^C" Enter;
cd ./src/main;
mv main /home/lrm/sideProjs/ambrosioBot;
tmux send-keys -t ambrosioBot "cd /home/lrm/sideProjs/ambrosioBot" Enter;
tmux send-keys -t ambrosioBot "TELEGRAM_API_KEY=${{ secrets.TELEGRAM_API_KEY }} ESP8266_ADDRESS_PORT=${{ secrets.ESP8266_ADDRESS_PORT }} ./main" Enter
