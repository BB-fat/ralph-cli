# Ralph CLI

> [中文文档](README_zh.md) | English

Ralph CLI is an AI Agent continuous execution engine that enables your AI agents to automatically iterate and persistently attempt until completing complex multi-step tasks.

## Core Features

Ralph's standout feature: **Make AI agents keep trying until they complete the task**

- **Automatic Iteration**: Set a maximum iteration count, and Ralph will automatically launch agents to complete stories one by one
- **Independent Sessions**: Each iteration launches a fresh agent instance to avoid context exhaustion
- **Progress Tracking**: Real-time tracking of the completion status of each user story
- **Failure Retry**: Automatically continue when errors occur until successful or maximum iterations are reached
- **Completion Signal**: Automatically stops when the agent outputs the `<promise>COMPLETE</promise>` signal

## Workflow

### Step 1: Install Ralph CLI

```bash
# Clone repository
git clone https://github.com/BB-fat/ralph-cli.git
cd ralph-cli

# Build project
cargo build --release

# Install to system (optional)
cargo install --path .
```

### Step 2: Check and Install AI Agents

Ensure at least one AI Agent CLI is installed in your system:

```bash
ralph detect
```

**Currently Supported AI Agents:**

| Agent | Command | Global Skills Directory |
|-------|---------|------------------------|
| Amp | `amp` | `~/.config/amp/skills/` |
| Claude Code | `claude` | `~/.claude/skills/` |
| CodeBuddy | `codebuddy` | `~/.codebuddy/skills/` |

Install Ralph Skills to your AI agents:

```bash
ralph install
```

This will install two skills:
- **`prd` skill**: Generate PRD (Product Requirements Document)
- **`ralph` skill**: Convert PRD to Ralph JSON format (`prd.json`)

### Step 3: Initialize Project

Create a Ralph workspace in your project:

```bash
cd your-project-directory
ralph init
```

Creates structure:
```
.
├── ralph/              # Ralph workspace
│   ├── prd.json       # Product requirements document (JSON format)
│   ├── progress.txt   # Current run progress log
│   ├── archive/      # Archived runs from previous branches
│   └── tasks/        # Generated PRD markdown files
```

### Step 4: Create PRD

Use the `/prd` skill in your AI agent:

```
Use prd skill, add user authentication system with email/password login
```

AI will generate a detailed PRD and save it to `ralph/tasks/prd-[feature-name].md`

### Step 5: Convert PRD to Ralph Format

Use the `/ralph` skill in your AI agent:

```
Use ralph skill, convert PRD to prd.json
```

AI will convert the PRD to `ralph/prd.json` format and break it down into executable user stories.

### Step 6: Run Ralph (Core Step)

Execute feature implementation:

```bash
ralph run
```

**Options:**
- `--tool`: Specify AI tool (amp/claude/codebuddy/auto)
- `--max-iterations`: Maximum number of iterations (default: 10)
- `--prd`: Path to prd.json (default: `./ralph/prd.json`)

### 🔄 How Ralph Run Works

```
Start → Load PRD → Analyze story status → Show progress
         ↓
    ┌────────────────────────────────────────┐
    │  Iteration Loop (keep trying until done) │
    │                                     │
    │  Iteration N / Max                  │
    │  - Launch new agent instance      │
    │  - Execute highest priority story │
    │  - Real-time streaming output (colored) │
    │  - Check completion signal       │
    │  - Not complete → Next iteration │
    │  - Complete   → Show summary     │
    └────────────────────────────────────────┘
```

**Key Features:**
- Each iteration is a fresh agent instance
- Track progress through `prd.json` and `progress.txt`
- Don't stop on errors, automatically retry
- Ctrl+C graceful shutdown, preserving completed work

## Use Cases

### Rapid Prototyping
```bash
cd new-project
ralph init
/prd create a blog system
/ralph convert PRD
ralph run
```

### Feature Enhancement
```bash
cd existing-project
ralph init
/prd add priority feature to existing task system
/ralph convert PRD
ralph run
```

### Configuration Management
```bash
# View all configurations
ralph config

# Set configuration values
ralph config set default_tool codebuddy
ralph config set max_iterations 15
ralph config set auto_archive true
```

**Configuration file:** `~/.config/ralph/config.toml`

## Configuration Options

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `default_tool` | string | `null` | Default AI tool (amp/claude/codebuddy) |
| `max_iterations` | integer | `10` | Maximum iterations per run |
| `auto_archive` | boolean | `true` | Automatically archive when switching branches |

## License

MIT

## Acknowledgments

This project is a Rust rewrite of the original [Ralph](https://github.com/snarktank/ralph) project by [jakedahn](https://github.com/snarktank).

## Related Projects

- [Amp](https://github.com/anthropics/amp) - Anthropic's AI coding assistant
- [Claude Code](https://github.com/anthropics/claude-code) - Claude's command-line tool
- [CodeBuddy](https://www.codebuddy.ai) - Intelligent programming assistant
