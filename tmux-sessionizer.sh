#!/usr/bin/env bash

./tmux-sessionizer.sh $(./target/release/tmux-sessionizer | fzf-tmux -p 80% --preview='
if string match -qr "^\[\*\]" {}
  set session_name (string replace -r "^\[\*\] " "" {})
  tmux capture-pane -ep -t "$session_name" 2>/dev/null; or echo "Failed to capture session preview"
else
  echo "Session not running"
end
')
