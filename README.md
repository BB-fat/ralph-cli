# Ralph CLI

Ralph CLI is an AI Agent aggregation tool that provides interactive skill installation, guided project initialization, and task management. It is a Rust-based rewrite of the original bash scripts, offering a more stable, efficient, and user-friendly experience.

## Features

- **Agent Auto-Detection**: Automatically detects installed AI Agent CLIs (Amp, Claude Code, CodeBuddy)
- **Interactive Skill Installation**: Select target agents and installation locations through an interactive interface
- **Guided Project Initialization**: Interactively create PRDs and project structures
- **Task Launch**: Launch AI Agent tasks with real-time output streaming
- **Configuration Management**: Manage user preferences (default tool, max iterations, etc.)
- **Colored Output**: Syntax highlighting (errors in red, warnings in yellow, success in green)
- **Auto-Archive**: Automatically archive progress when switching branches

## Supported AI Agents

| Agent | Command | Global Skills Directory |
|-------|---------|------------------------|
| Amp | `amp` | `~/.config/amp/skills/` |
| Claude Code | `claude` | `~/.claude/skills/` |
| CodeBuddy | `codebuddy` | `~/.codebuddy/skills/` |

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd ralph-cli

# Build the project
cargo build --release

# Install to system (optional)
cargo install --path .
```

## Usage

### Quick Start

1. **Install and setup**
2. **Initialize your project**
3. **Create a PRD (Product Requirements Document)**
4. **Run Ralph to implement the feature**

See the [Complete Workflow](#complete-workflow) section below for detailed step-by-step instructions.

### Show Help

```bash
ralph --help
ralph --version
```

### Detect Installed Agents

Check which AI Agent CLIs are available on your system:

```bash
ralph detect
```

This will show the status of Amp, Claude Code, and CodeBuddy.

### Install Skills

Interactively select target agents and installation locations:

```bash
ralph install
```

Supported installation targets:
- Project local: `./scripts/ralph/skills/`
- Amp global: `~/.config/amp/skills/`
- Claude global: `~/.claude/skills/`
- CodeBuddy global: `~/.codebuddy/skills/`

### Configuration Management

View configuration:
```bash
ralph config
```

Set configuration values:
```bash
ralph config set default_tool codebuddy
ralph config set max_iterations 10
ralph config set auto_archive true
```

Configuration file location: `~/.config/ralph/config.toml`

### Project Status

```bash
ralph status
```

### Archive Management

```bash
ralph archive
```

## Project Structure

```
ralph-cli/
├── Cargo.toml          # Rust project configuration
├── src/
│   ├── main.rs         # Main entry point
│   ├── cli.rs          # CLI argument parsing
│   ├── config.rs       # Configuration management
│   ├── prd.rs          # PRD data structures
│   ├── agent.rs        # Agent detection
│   ├── error.rs        # Error handling
│   ├── templates.rs    # Template management
│   └── commands/       # Command implementations
│       ├── init.rs
│       ├── run.rs
│       ├── install.rs
│       ├── config.rs
│       └── detect.rs
└── ralph/              # Ralph workspace (created by init)
    ├── prd.json
    ├── progress.txt
    ├── archive/
    └── tasks/
```

## Tech Stack

- **CLI Parsing**: [clap](https://crates.io/crates/clap) v4 with derive feature
- **Interactive Prompts**: [dialoguer](https://crates.io/crates/dialoguer)
- **Terminal Styling**: [colored](https://crates.io/crates/colored) + [console](https://crates.io/crates/console)
- **Async Runtime**: [tokio](https://crates.io/crates/tokio)
- **Configuration**: [dirs](https://crates.io/crates/dirs) + [toml](https://crates.io/crates/toml)
- **Serialization**: [serde](https://crates.io/crates/serde) + [serde_json](https://crates.io/crates/serde_json)
- **Date/Time**: [chrono](https://crates.io/crates/chrono)

## Development

```bash
# Run development version
cargo run

# Run tests
cargo test

# Run strict linting
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default_tool` | string | `null` | Default AI tool to use |
| `max_iterations` | integer | `10` | Default maximum iterations for task execution |
| `auto_archive` | boolean | `true` | Auto-archive history on branch switch |

## License

MIT

## Acknowledgments

This project is a Rust rewrite of the original [Ralph](https://github.com/jakedahn/ralph) project by [jakedahn](https://github.com/jakedahn). The original Ralph was a collection of bash scripts that provided AI-assisted development workflows. This Rust version aims to provide the same functionality with improved stability, performance, and user experience.

Special thanks to the original author for the innovative concept and workflow design.

## Related Projects

- [Amp](https://github.com/anthropics/amp) - Anthropic's AI coding assistant
- [Claude Code](https://github.com/anthropics/claude-code) - Claude's command-line tool
- [CodeBuddy](https://www.codebuddy.ai) - Intelligent programming assistant
