#!/usr/bin/env bash

./target/release/tmux-sessionizer $(./target/release/tmux-sessionizer | fzf-tmux -p 80%)
