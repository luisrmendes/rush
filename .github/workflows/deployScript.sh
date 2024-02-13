#!/bin/bash

if tmux list-windows | grep -q 'rushBot' 
then
    echo "Session exists"
else
    echo "Session does not exist, creating sesion";
    tmux new-session -s rushBot;
fi

tmux send-keys -t rushBot "^C" Enter;
cd ./src/main;
mv main /home/lrm/sideProjs/ambrosioBot;
tmux send-keys -t rushBot "cd /home/lrm/sideProjs/ambrosioBot" Enter;
tmux send-keys -t rushBot "TELEGRAM_API_KEY=";
tmux send-keys -t rushBot $1;
tmux send-keys -t rushBot " ESP8266_ADDRESS_PORT=";
tmux send-keys -t rushBot $2;
tmux send-keys -t rushBot " SYSTEM_1_ADDRESS=";
tmux send-keys -t rushBot $3;
tmux send-keys -t rushBot " SYSTEM_2_ADDRESS=";
tmux send-keys -t rushBot $4;
tmux send-keys -t rushBot " ./main" Enter;
