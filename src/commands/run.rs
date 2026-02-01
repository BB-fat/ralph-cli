use chrono::Local;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::signal;

use crate::agent::{detect_agents, is_command_available};
use crate::config::Config;
use crate::error::{RalphError, RalphResult};
use crate::prd::Prd;
use crate::templates::get_agent_prompt;

/// Check for legacy files in old locations and offer migration
fn check_and_offer_migration() -> RalphResult<()> {
    let legacy_prd = Path::new("./prd.json");
    let legacy_progress = Path::new("./progress.txt");
    let new_dir = Path::new("./ralph");
    let new_prd = new_dir.join("prd.json");

    // Check if legacy files exist and new location doesn't
    if legacy_prd.exists() && !new_prd.exists() {
        println!("{}", "═══════════════════════════════════════".yellow());
        println!("{}", "  Legacy files detected!".yellow().bold());
        println!("{}", "═══════════════════════════════════════".yellow());
        println!();
        println!("Found prd.json in the old location (root directory).");
        println!("Ralph now stores all project files in the 'ralph/' directory.");
        println!();
        println!("Would you like to migrate your files? [Y/n]");

        // Read user input
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(RalphError::Io)?;

        let input = input.trim().to_lowercase();
        if input.is_empty() || input == "y" || input == "yes" {
            // Perform migration
            fs::create_dir_all(new_dir)?;

            // Migrate prd.json
            if legacy_prd.exists() {
                fs::copy(legacy_prd, &new_prd)?;
                fs::remove_file(legacy_prd)?;
                println!("  ✓ Migrated prd.json → ralph/prd.json");
            }

            // Migrate progress.txt
            if legacy_progress.exists() {
                let new_progress = new_dir.join("progress.txt");
                fs::copy(legacy_progress, &new_progress)?;
                fs::remove_file(legacy_progress)?;
                println!("  ✓ Migrated progress.txt → ralph/progress.txt");
            }

            // Migrate archive directory if it exists
            let legacy_archive = Path::new("./archive");
            if legacy_archive.exists() && legacy_archive.is_dir() {
                let new_archive = new_dir.join("archive");
                fs::create_dir_all(&new_archive)?;
                // Move all contents from old archive to new archive
                for entry in fs::read_dir(legacy_archive)? {
                    let entry = entry?;
                    let src = entry.path();
                    let dst = new_archive.join(entry.file_name());
                    fs::rename(&src, &dst)?;
                }
                fs::remove_dir(legacy_archive)?;
                println!("  ✓ Migrated archive/ → ralph/archive/");
            }

            println!();
            println!("{}", "Migration complete!".green().bold());
            println!();
            println!("Please run your command again.");
            std::process::exit(0);
        } else {
            println!("Migration skipped. Please manually move your files to the 'ralph/' directory.");
            return Err(RalphError::Other(
                "Migration required. Run again and accept migration, or manually move files to ralph/".to_string()
            ));
        }
    }

    Ok(())
}

/// Run the Ralph task execution command
pub async fn run_run(
    tool: String,
    max_iterations: Option<u32>,
    prd_path: String,
) -> RalphResult<()> {
    // Load configuration
    let config = Config::load()?;

    // Determine max iterations
    let max_iter = max_iterations.or(config.max_iterations).unwrap_or(10);

    // Check for legacy files and offer migration
    check_and_offer_migration()?;

    // Get the directory containing prd.json (the ralph working directory)
    let prd_file_path = PathBuf::from(&prd_path);
    let ralph_dir = prd_file_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    // Ensure the ralph directory exists
    if !ralph_dir.exists() {
        return Err(RalphError::Other(format!(
            "Ralph directory does not exist: {}. Run 'ralph init' to initialize.",
            ralph_dir.display()
        )));
    }

    // Load PRD
    let prd = Prd::from_file(&prd_path).map_err(|e| {
        RalphError::Other(format!("Failed to load PRD from {}: {}", prd_path, e))
    })?;

    // Determine which tool to use
    let tool_cmd = determine_tool(&tool, &config)?;

    // Display startup information
    println!("{}", "Ralph Task Runner".bold().cyan());
    println!("{}", "=================".cyan());
    println!();
    println!("Project: {}", prd.project.bold());
    println!("Branch: {}", prd.branch_name().cyan());
    println!("Tool: {}", tool_cmd.cyan());
    println!();
    println!(
        "Progress: {}/{} stories completed",
        prd.completed_stories().to_string().green(),
        prd.total_stories()
    );
    println!();

    // Check if all stories are complete
    if prd.pending_stories() == 0 {
        println!("{}", "All stories are complete!".green().bold());
        return Ok(());
    }

    // Handle archive logic if branch changed
    handle_archive(&ralph_dir, &prd)?;

    // Initialize progress file if it doesn't exist
    let progress_file = ralph_dir.join("progress.txt");
    init_progress_file(&progress_file)?;

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    tokio::spawn(async move {
        if signal::ctrl_c().await.is_ok() {
            println!();
            println!("{}", "Received interrupt signal, stopping...".yellow());
            r.store(false, Ordering::SeqCst);
        }
    });

    // Run iterations
    let mut current_iteration = 1;

    while current_iteration <= max_iter && running.load(Ordering::SeqCst) {
        println!(
            "\n{} {} / {}",
            "Iteration".bold(),
            current_iteration,
            max_iter
        );
        println!("{}", "-".repeat(40).dimmed());

        // Run the agent
        let completed = run_agent_iteration(&tool_cmd, &ralph_dir, running.clone()).await?;

        if completed {
            println!();
            println!("{}", "✓ Agent signaled completion!".green().bold());
            break;
        }

        current_iteration += 1;
    }

    // Display summary
    println!();
    println!("{}", "=================".cyan());
    println!("{}", "Run Summary".bold().cyan());
    println!("{}", "=================".cyan());
    println!(
        "Iterations completed: {}/{}",
        (current_iteration - 1).min(max_iter),
        max_iter
    );

    // Reload PRD to get updated status
    let final_prd = Prd::from_file(&prd_path).unwrap_or(prd);
    println!(
        "Stories completed: {}/{}",
        final_prd.completed_stories(),
        final_prd.total_stories()
    );

    if !running.load(Ordering::SeqCst) {
        println!("{}", "Run interrupted by user".yellow());
    } else if current_iteration > max_iter {
        println!("{}", "Maximum iterations reached".yellow());
    }

    Ok(())
}

/// Handle archive logic when branch changes
fn handle_archive(ralph_dir: &Path, prd: &Prd) -> RalphResult<()> {
    let last_branch_file = ralph_dir.join(".last-branch");
    let current_branch = &prd.branch_name;

    // Check if there's a previous branch to archive
    if last_branch_file.exists() {
        let last_branch = fs::read_to_string(&last_branch_file)?;
        let last_branch = last_branch.trim();

        if !last_branch.is_empty() && last_branch != current_branch {
            // Branch changed, archive the previous run
            let date = Local::now().format("%Y-%m-%d").to_string();
            let folder_name = last_branch.strip_prefix("ralph/").unwrap_or(last_branch);
            let archive_dir = ralph_dir.join("archive").join(format!("{}-{}", date, folder_name));

            println!(
                "Archiving previous run: {} -> {}",
                last_branch.cyan(),
                archive_dir.display()
            );

            fs::create_dir_all(&archive_dir)?;

            // Copy prd.json if it exists
            let prd_file = ralph_dir.join("prd.json");
            if prd_file.exists() {
                fs::copy(&prd_file, archive_dir.join("prd.json"))?;
            }

            // Copy progress.txt if it exists
            let progress_file = ralph_dir.join("progress.txt");
            if progress_file.exists() {
                fs::copy(&progress_file, archive_dir.join("progress.txt"))?;
            }

            // Reset progress file for new run
            init_progress_file(&progress_file)?;
        }
    }

    // Track current branch
    fs::write(&last_branch_file, current_branch)?;

    Ok(())
}

/// Initialize progress file if it doesn't exist
fn init_progress_file(progress_file: &Path) -> RalphResult<()> {
    if !progress_file.exists() {
        let content = format!(
            "# Ralph Progress Log\nStarted: {}\n---\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        fs::write(progress_file, content)?;
    }
    Ok(())
}

/// Determine which tool command to use
pub fn determine_tool(tool: &str, config: &Config) -> Result<String, crate::error::RalphError> {
    match tool {
        "auto" => {
            // Try to use default_tool from config, otherwise auto-detect
            if let Some(ref default) = config.default_tool {
                // Verify the tool is available
                if is_command_available(default) {
                    Ok(default.clone())
                } else {
                    // Try to detect any available agent
                    let detected = detect_agents();
                    if let Some(first) = detected.first() {
                        Ok(first.command().to_string())
                    } else {
                        Err(RalphError::Other(
                            "No AI agent CLI detected. Please install Amp, Claude Code, or CodeBuddy.".to_string()
                        ))
                    }
                }
            } else {
                // Auto-detect
                let detected = detect_agents();
                if let Some(first) = detected.first() {
                    Ok(first.command().to_string())
                } else {
                    Err(RalphError::Other(
                        "No AI agent CLI detected. Please install Amp, Claude Code, or CodeBuddy.".to_string()
                    ))
                }
            }
        }
        "amp" => Ok("amp".to_string()),
        "claude" => Ok("claude".to_string()),
        "codebuddy" => Ok("codebuddy".to_string()),
        _ => Ok(tool.to_string()), // Allow custom tool commands
    }
}

/// Run a single agent iteration
async fn run_agent_iteration(
    tool_cmd: &str,
    ralph_dir: &Path,
    running: Arc<AtomicBool>,
) -> RalphResult<bool> {
    // Get the embedded prompt content
    let prompt_content = get_agent_prompt();

    // Build the command based on the tool
    let mut cmd = TokioCommand::new(tool_cmd);

    // Set the working directory to the ralph directory
    cmd.current_dir(ralph_dir);

    // Configure command based on tool type
    match tool_cmd {
        "amp" => {
            // amp: read skill file from stdin with --dangerously-allow-all flag
            cmd.arg("--dangerously-allow-all");
            cmd.stdin(std::process::Stdio::piped());
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        }
        "claude" => {
            // claude: use --dangerously-skip-permissions and --print, read from stdin
            cmd.arg("--dangerously-skip-permissions");
            cmd.arg("--print");
            cmd.stdin(std::process::Stdio::piped());
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        }
        "codebuddy" => {
            // codebuddy: use -p --dangerously-skip-permissions --tools default, read from stdin
            cmd.arg("-p");
            cmd.arg("--dangerously-skip-permissions");
            cmd.arg("--tools");
            cmd.arg("default");
            cmd.stdin(std::process::Stdio::piped());
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        }
        _ => {
            // For custom tools, use basic stdin redirection
            cmd.stdin(std::process::Stdio::piped());
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
        }
    }

    // Spawn the process
    let mut child = cmd.spawn().map_err(|e| {
        RalphError::Other(format!("Failed to spawn {}: {}", tool_cmd, e))
    })?;

    // Write prompt content to stdin
    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin.write_all(prompt_content.as_bytes()).await.map_err(|e| {
            RalphError::Other(format!("Failed to write to stdin: {}", e))
        })?;
        // Close stdin to signal EOF
        // stdin is dropped here, which closes the pipe
    }

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let mut found_complete = false;

    // Stream output with color highlighting
    loop {
        if !running.load(Ordering::SeqCst) {
            // User interrupted, kill the child process
            let _ = child.kill().await;
            break;
        }

        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        // Check for completion signal
                        if line.contains("<promise>COMPLETE</promise>") {
                            found_complete = true;
                        }
                        // Print with color highlighting
                        println!("{}", colorize_output(&line));
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        // Print stderr in red
                        eprintln!("{}", line.red());
                    }
                    Ok(None) => break,
                    Err(_) => break,
                }
            }
        }
    }

    // Wait for the process to complete
    let status: std::process::ExitStatus = child.wait().await.map_err(RalphError::Io)?;

    if !status.success() && running.load(Ordering::SeqCst) {
        eprintln!(
            "{}",
            format!(
                "Warning: {} exited with status: {:?}",
                tool_cmd,
                status.code()
            )
            .yellow()
        );
    }

    Ok(found_complete)
}

/// Apply color highlighting to output lines
pub fn colorize_output(line: &str) -> String {
    // Highlight common patterns
    if line.contains("Error") || line.contains("error") || line.contains("ERROR") {
        line.red().to_string()
    } else if line.contains("Warning") || line.contains("warning") || line.contains("WARNING") {
        line.yellow().to_string()
    } else if line.contains("Success") || line.contains("success") || line.contains('✓') {
        line.green().to_string()
    } else if line.contains("<promise>COMPLETE</promise>") {
        line.bright_green().bold().to_string()
    } else {
        line.to_string()
    }
}
