# Ralph CLI

Ralph CLI 是一个 AI Agent 持续执行引擎，让你的 AI 代理人自动迭代、持续尝试，直到完成复杂的多步骤任务。

## 核心特性

Ralph 最大的特点：**让 AI agent 不断尝试，直到完成任务**

- **自动迭代**：设置最大迭代次数，Ralph 会自动启动 agent 逐个完成故事
- **独立会话**：每次迭代启动全新的 agent 实例，避免上下文耗尽
- **进度跟踪**：实时追踪每个用户故事的完成状态
- **失败重试**：遇到错误时自动继续，直到成功或达到最大迭代次数
- **完成信号**：Agent 输出 `<promise>COMPLETE</promise>` 信号时自动停止

## 安装

### 方式 1：通过 npm 安装（推荐）

```bash
# 全局安装
npm install -g ralph-cli

# 或使用 npx 无需安装
npx ralph-cli <command>
```

### 方式 2：从源码构建

```bash
# 克隆仓库
git clone https://github.com/BB-fat/ralph-cli.git
cd ralph-cli

# 构建项目
cargo build --release

# 安装到系统（可选）
cargo install --path .
```

### 离线安装

对于没有互联网访问的环境：

```bash
# 首先从 GitHub Releases 下载适合你平台的二进制文件
# 然后使用本地二进制文件路径安装
npm install -g ralph-cli --ralph-binary-path=/path/to/ralph

# 或使用环境变量
RALPH_BINARY_PATH=/path/to/ralph npm install -g ralph-cli
```

## 工作流

### 步骤 1: 验证安装

```bash
ralph --version
```

### 步骤 2: 检查和安装 AI Agents

确保系统中已安装至少一个 AI Agent CLI：

```bash
ralph detect
```

**目前支持的 AI Agents:**

| Agent | 命令 | 全局技能目录 |
|-------|---------|-------------|
| Amp | `amp` | `~/.config/amp/skills/` |
| Claude Code | `claude` | `~/.claude/skills/` |
| CodeBuddy | `codebuddy` | `~/.codebuddy/skills/` |

安装 Ralph Skills 到你的 AI agents：

```bash
ralph install
```

这将安装两个技能：
- **`prd` 技能**：生成 PRD（产品需求文档）
- **`ralph` 技能**：将 PRD 转换为 Ralph JSON 格式（`prd.json`）

### 步骤 3: 初始化项目

在你的项目中创建 Ralph 工作空间：

```bash
cd your-project-directory
ralph init
```

创建结构：
```
.
├── ralph/              # Ralph 工作空间
│   ├── prd.json       # 产品需求文档（JSON 格式）
│   ├── progress.txt   # 当前运行的进度日志
│   ├── archive/      # 来自之前分支的归档运行
│   └── tasks/        # 生成的 PRD markdown 文件
```

### 步骤 4: 创建 PRD

使用 AI agent 中的 `/prd` 技能：

```
使用  prd skill, 添加用户认证系统，支持邮箱/密码登录
```

AI 将生成详细的 PRD 并保存到 `ralph/tasks/prd-[feature-name].md`

### 步骤 5: 转换 PRD 为 Ralph 格式

使用 AI agent 中的 `/ralph` 技能：

```
使用 ralph skill, 转换 PRD 为 prd.json
```

AI 将把 PRD 转换为 `ralph/prd.json` 格式，并分解为可执行的用户故事。

### 步骤 6: 运行 Ralph（核心步骤）

执行功能实现：

```bash
ralph run
```

**选项：**
- `--tool`: 指定 AI 工具（amp/claude/codebuddy/auto）
- `--max-iterations`: 最大迭代次数（默认：10）
- `--prd`: prd.json 的路径（默认：`./ralph/prd.json`）

### 🔄 Ralph Run 的工作原理

```
启动 → 加载 PRD → 分析故事状态 → 显示进度
         ↓
    ┌────────────────────────────────────────┐
    │  迭代循环（持续尝试，直到完成）        │
    │                                     │
    │  Iteration N / Max                  │
    │  - 启动新的 agent 实例            │
    │  - 执行最高优先级的待办故事         │
    │  - 实时输出流式显示（带颜色）      │
    │  - 检查完成信号                  │
    │  - 未完成 → 继续下一个迭代         │
    │  - 完成   → 显示总结              │
    └────────────────────────────────────────┘
```

**关键特性：**
- 每次迭代都是全新的 agent 实例
- 通过 `prd.json` 和 `progress.txt` 跟踪进度
- 遇到错误不停止，自动重试
- Ctrl+C 优雅停止，保留已完成工作

## 使用场景

### 快速原型开发
```bash
cd new-project
ralph init
/prd 创建一个博客系统
/ralph 转换 PRD
ralph run
```

### 功能增强
```bash
cd existing-project
ralph init
/prd 为现有任务系统添加优先级功能
/ralph 转换 PRD
ralph run
```

### 配置管理
```bash
# 查看所有配置
ralph config

# 设置配置值
ralph config set default_tool codebuddy
ralph config set max_iterations 15
ralph config set auto_archive true
```

**配置文件：** `~/.config/ralph/config.toml`

## 配置选项

| 设置 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `default_tool` | string | `null` | 默认 AI 工具（amp/claude/codebuddy） |
| `max_iterations` | integer | `10` | 每次运行的最大迭代次数 |
| `auto_archive` | boolean | `true` | 切换分支时自动归档 |

## 故障排除

### 安装问题

**npm 安装期间二进制文件下载失败**

```
npm ERR! Installation failed: Error: Failed to download: 404
```

**解决方案：**
1. 检查网络连接
2. 确认 GitHub 上存在该版本的发布
3. 使用离线安装方式，使用预下载的二进制文件：
   ```bash
   npm install -g ralph-cli --ralph-binary-path=/path/to/ralph
   ```

**运行 ralph 时权限被拒绝**

```bash
# 在 Linux/macOS 上，确保二进制文件可执行
chmod +x $(npm root -g)/ralph-cli/bin/ralph
```

**安装后找不到二进制文件**

```bash
# 重新安装包
npm uninstall -g ralph-cli
npm install -g ralph-cli

# 或手动从 GitHub Releases 下载适合你平台的二进制文件
# 并将其放入 npm 包的 bin/ 目录中
```

### 平台支持

Ralph CLI npm 包支持：
- **macOS**: Intel (x64) 和 Apple Silicon (arm64)
- **Linux**: x64 和 ARM64

Windows 目前不支持通过 npm 安装。请使用 WSL 或从源码构建。

## License

MIT

## Acknowledgments

This project is a Rust rewrite of the original [Ralph](https://github.com/snarktank/ralph) project by [jakedahn](https://github.com/snarktank).

## Related Projects

- [Amp](https://github.com/anthropics/amp) - Anthropic's AI coding assistant
- [Claude Code](https://github.com/anthropics/claude-code) - Claude's command-line tool
- [CodeBuddy](https://www.codebuddy.ai) - Intelligent programming assistant
