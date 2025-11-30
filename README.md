# tmux-sessionizer

A Rust-based tmux session manager for quickly creating, managing, and attaching to tmux sessions across different directories and git projects.

## Overview

tmux-sessionizer helps you manage tmux sessions through configuration files. Define your project sessions once, then quickly create and attach to them from anywhere.

## Planned Features

- **Create Sessions**: Start new tmux sessions in specified directories or git projects
- **Attach to Sessions**: Quickly attach to existing sessions
- **Add Sessions**: Define new sessions via configuration files
- **Configuration Support**: Sessions can specify:
  - Working directory or git project location
  - Custom shell commands to run on creation

## Development

```bash
# Build the project
cargo build

# Run in development
cargo run
```

## License

MIT - See [LICENSE](LICENSE) for details.

