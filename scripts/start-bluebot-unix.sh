#!/bin/bash
tmux new-session -d -s bluebot

tmux send-keys -t bluebot 'tmux new-window -n Start ' ENTER
tmux send-keys -t bluebot "tmux send-keys -t Start 'cargo run' ENTER" ENTER

## Start a la fenêtre 0
tmux send-keys -t bluebot ENTER

## Envoi des commandes et attache la session
tmux send-keys -t bluebot
tmux attach -t bluebot
