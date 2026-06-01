use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use clap::{Parser, Subcommand};
use clap_complete::{Shell, generate};
use directories::ProjectDirs;

#[derive(Parser)]
#[command(name = "bunsh", about = "Edit and run TypeScript scripts with bun")]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new script, open in $EDITOR, and run with bun
    New,
    /// Open an existing script by name or path, then run with bun
    Run {
        /// Script name, partial match, or path
        script: String,
    },
    /// List saved scripts
    #[command(alias = "ls")]
    List,
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
    /// Delete a saved script
    Rm {
        /// Script name, partial match, or path
        script: String,
    },
    /// Print shell init script (wrapper function for history rewriting)
    Init {
        /// Shell to generate init script for
        shell: Shell,
    },
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "bunsh").expect("could not determine home directory")
}

fn scripts_dir() -> PathBuf {
    project_dirs().data_dir().to_path_buf()
}

fn new_script_path() -> PathBuf {
    let now = chrono::Local::now();
    let name = now.format("%Y-%m-%d-%H%M%S.ts").to_string();
    scripts_dir().join(name)
}

fn resolve_script(arg: &str) -> Option<PathBuf> {
    let p = PathBuf::from(arg);
    if p.exists() {
        return Some(p);
    }

    let dir = scripts_dir();
    let exact = dir.join(arg);
    if exact.exists() {
        return Some(exact);
    }
    let with_ext = dir.join(format!("{arg}.ts"));
    if with_ext.exists() {
        return Some(with_ext);
    }

    // Substring match (newest first)
    if let Ok(entries) = fs::read_dir(&dir) {
        let mut matches: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.contains(arg))
            })
            .collect();
        matches.sort();
        matches.reverse();
        return matches.into_iter().next();
    }

    None
}

fn list_scripts() {
    let dir = scripts_dir();
    if !dir.exists() {
        println!("No scripts yet.");
        return;
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .expect("failed to read scripts dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("ts"))
        .collect();
    entries.sort();
    if entries.is_empty() {
        println!("No scripts yet.");
        return;
    }
    for entry in &entries {
        println!("{}", entry.display());
    }
}

fn open_and_run(path: &PathBuf) -> ExitCode {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "nano".to_string());

    let status = Command::new(&editor)
        .arg(path)
        .status()
        .expect("failed to open editor");

    if !status.success() {
        eprintln!("editor exited with {status}");
        return ExitCode::FAILURE;
    }

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("file not found after editor closed");
            let _ = fs::remove_file(path);
            return ExitCode::FAILURE;
        }
    };

    let default_imports = include_str!("default_imports.ts");
    let is_unmodified = contents.trim().is_empty() || contents.trim() == default_imports.trim();

    if is_unmodified {
        eprintln!("unmodified file, skipping execution");
        let _ = fs::remove_file(path);
        return ExitCode::FAILURE;
    }

    let display_path = path.display();
    // Machine-readable line for shell wrapper to rewrite history
    eprintln!("__bunsh_script__:{display_path}");
    println!("--- running {display_path} ---");
    let run = Command::new("bun")
        .arg("run")
        .arg(path)
        .status()
        .expect("failed to run bun");

    println!("--- {display_path} ---");
    if run.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(run.code().unwrap_or(1) as u8)
    }
}

fn ensure_tsconfig(dir: &std::path::Path) {
    let tsconfig = dir.join("tsconfig.json");
    if !tsconfig.exists() {
        let config = include_str!("tsconfig.json");
        fs::write(&tsconfig, config).expect("failed to write tsconfig.json");

        let node_modules = dir.join("node_modules");
        if !node_modules.join("@types/bun").exists() {
            let _ = Command::new("bun")
                .args(["add", "-d", "@types/bun"])
                .current_dir(dir)
                .output();
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Cmd::List) => {
            list_scripts();
            ExitCode::SUCCESS
        }
        Some(Cmd::Run { script }) => match resolve_script(&script) {
            Some(path) => open_and_run(&path),
            None => {
                eprintln!("no script matching '{script}' found");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::Rm { script }) => match resolve_script(&script) {
            Some(path) => {
                println!("removing {}", path.display());
                fs::remove_file(&path).expect("failed to remove script");
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("no script matching '{script}' found");
                ExitCode::FAILURE
            }
        },
        Some(Cmd::Completions { shell }) => {
            let mut cmd = <Cli as clap::CommandFactory>::command();
            generate(shell, &mut cmd, "bunsh", &mut io::stdout());
            ExitCode::SUCCESS
        }
        Some(Cmd::Init { shell }) => {
            match shell {
                Shell::Fish => print!("{}", include_str!("shell_inits/bunsh.fish")),
                Shell::Bash => print!("{}", include_str!("shell_inits/bunsh.bash")),
                Shell::Zsh => print!("{}", include_str!("shell_inits/bunsh.zsh")),
                _ => eprintln!("init not yet supported for {shell}"),
            }
            ExitCode::SUCCESS
        }
        Some(Cmd::New) | None => {
            let dir = scripts_dir();
            fs::create_dir_all(&dir).expect("failed to create scripts dir");
            ensure_tsconfig(&dir);
            let path = new_script_path();
            let default_content = include_str!("default_imports.ts");
            fs::write(&path, default_content).expect("failed to create script file");
            open_and_run(&path)
        }
    }
}
