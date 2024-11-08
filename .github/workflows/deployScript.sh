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

# Kill all instances of the process by name
killall rush

# Wait for all processes to be terminated
while pgrep rush > /dev/null; do sleep 1; done

# Notify when the processes are terminated
echo "All instances of 'rush' have been terminated."

tmuxSeshCommand=""
tmuxSeshCommand="${tmuxSeshCommand} ./whisperSetup.sh && "
tmuxSeshCommand="${tmuxSeshCommand} TELOXIDE_TOKEN=$1 ESP8266_ADDRESS_PORT=$2 SYRINX_VARS=$3 SNOWDOG_VARS=$4 CYGNUS_VARS=$5 PI3_VARS=$6"
tmuxSeshCommand="${tmuxSeshCommand} RUST_LOG=trace target/release/rush"

tmux send-keys -t $tmuxSessionName "${tmuxSeshCommand}" Enter
