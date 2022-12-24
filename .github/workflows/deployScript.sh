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
tmux send-keys -t ambrosioBot "TELEGRAM_API_KEY=";
tmux send-keys -t ambrosioBot $1;
tmux send-keys -t ambrosioBot " ESP8266_ADDRESS_PORT=";
tmux send-keys -t ambrosioBot $2;
tmux send-keys -t ambrosioBot " SYSTEM_1_ADDRESS=";
tmux send-keys -t ambrosioBot $3;
tmux send-keys -t ambrosioBot " SYSTEM_2_ADDRESS=";
tmux send-keys -t ambrosioBot $4;
tmux send-keys -t ambrosioBot " ./main" Enter;
