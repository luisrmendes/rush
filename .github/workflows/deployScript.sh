#!/bin/bash
tmuxSessionName="rushServer"

pwd=$(pwd)

# Create tmux session if its not already created
if tmux list-sessions | grep -w "$tmuxSessionName" 2>/dev/null 1>&2; then
    echo "Session exists"
else
    echo "Session does not exist, creating session \"$tmuxSessionName\""
    tmux new-session -s $tmuxSessionName -d
fi

tmux send-keys -t $tmuxSessionName "^C" Enter

tmuxSeshCommand=""
tmuxSeshCommand="${tmuxSeshCommand} cd /home/lrm/sideProjs/rush && git pull && cargo build --release && "
tmuxSeshCommand="${tmuxSeshCommand} TELOXIDE_TOKEN=$1 ESP8266_ADDRESS_PORT=$2 SYSTEM0_USER=$3 SYSTEM0_IP_ADDR=$4 SYSTEM1_USER=$5 SYSTEM1_IP_ADDR=$6 SYSTEM2_USER=$7 SYSTEM2_IP_ADDR=$8 SYSTEM2_MAC=$9"
tmuxSeshCommand="${tmuxSeshCommand} RUST_LOG=warn target/release/rush Enter"

tmux send-keys -t $tmuxSessionName "${tmuxSeshCommand}" Enter
