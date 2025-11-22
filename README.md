# TMUX Sessionizer

A simple and lightweight session management tool for TMUX.

## Config Reference Table

  | Field                   | Type        | Default | Effect                                  |
  |-------------------------|-------------|---------|-----------------------------------------|
  | directories             | Vec<String> | []      | Parent directories to scan for sessions |
  | max_depth               | usize       | 1       | How deep to recurse into directories    |
  | display_tilde           | bool        | true    | Show ~/... instead of /Users/name/...   |
  | exclude_current_session | bool        | true    | Hide current tmux session from list     |

