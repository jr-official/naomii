mod ports;
mod server;
mod fs;

use std::io::{self, Write};
use server::setup;
use crossterm::{
    style::{PrintStyledContent, Stylize},
    ExecutableCommand,
    terminal,
    cursor,
};
use ports::get_free_port;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Naomii", about = "Naomii Server Manager CLI")]
struct CliArgs {
    #[arg(short, long)]
    port: Option<u32>,

    #[arg(short = 'l', long)]
    local_server_path: Option<String>,

    #[arg(short = 's', long)]
    server_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    if std::env::args().len() > 1 {
        let cli_args = CliArgs::parse();

        if cli_args.port.is_some() && cli_args.local_server_path.is_some() && cli_args.server_path.is_some() {
            print_section_header(&mut stdout, "Server Setup (Exec Mode)")?;

            let port = cli_args.port.unwrap();
            let local_server_path = cli_args.local_server_path.unwrap();
            let server_path = cli_args.server_path.unwrap();

            print_status(&mut stdout, &format!("Setting up NaomiiRouter on port {}...", port))?;

            match setup(port, server_path.clone(), local_server_path.clone()).await {
                Ok(_) => {
                    print_success(&mut stdout, &format!(
                        "Router successfully started on port {} (path: {})",
                        port, server_path
                    ))?;
                }
                Err(_) => {
                    print_warning(&mut stdout, "Port unavailable, selecting a free port instead...")?;
                    let free_port = get_free_port();
                    match setup(free_port, server_path.clone(), local_server_path.clone()).await {
                        Ok(_) => {
                            print_success(&mut stdout, &format!(
                                "Router started on port {} (path: {})",
                                free_port, server_path
                            ))?;
                        }
                        Err(_) => {
                            print_error(&mut stdout, &format!(
                                "Could not set up NaomiiRouter on port {} (path: {})",
                                free_port, server_path
                            ))?;
                        }
                    }
                }
            }

            return Ok(());
        } else {
            print_error(&mut stdout, "All flags -p, -l, and -s must be provided in exec mode")?;
            return Ok(());
        }
    }

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    print_banner(&mut stdout)?;

    stdout.execute(PrintStyledContent("\nCommands start with ".grey()))?;
    stdout.execute(PrintStyledContent("!".yellow().bold()))?;
    stdout.execute(PrintStyledContent(" — type ".grey()))?;
    stdout.execute(PrintStyledContent("help".yellow().bold()))?;
    stdout.execute(PrintStyledContent(" to see available commands.\n\n".grey()))?;

    loop {
        stdout.execute(PrintStyledContent(">> ".magenta().bold()))?;
        stdout.flush()?;

        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            print_error(&mut stdout, "Could not read command")?;
            continue;
        }

        let cmd = cmd.trim();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "!help" | "help" | "?" => print_help(&mut stdout)?,

            "!startup" | "startup" | "start" => {
                stdout.execute(PrintStyledContent("Starting Naomii installation...\n".cyan().bold()))?;
                todo!()
            }

            "!setnew" | "set" | "new" | "server" => {
                print_section_header(&mut stdout, "Server Setup")?;

                stdout.execute(PrintStyledContent("Enter a port for ".white()))?;
                stdout.execute(PrintStyledContent("NaomiiRouter".cyan().bold()))?;
                stdout.execute(PrintStyledContent(": ".white()))?;
                stdout.flush()?;

                let mut custom_port = String::new();
                if io::stdin().read_line(&mut custom_port).is_err() {
                    print_error(&mut stdout, "Unable to read port")?;
                    continue;
                }

                let port = match custom_port.trim().parse::<u32>() {
                    Ok(num) if num > 0 && num < 65536 => num,
                    _ => {
                        print_error(&mut stdout, "Invalid port number (1-65535)")?;
                        continue;
                    }
                };

                stdout.execute(PrintStyledContent("Where does the bin for this server live?: ".blue().bold()))?;
                let mut local_server_path = String::new();
                if io::stdin().read_line(&mut local_server_path).is_err() {
                    print_error(&mut stdout, "Unable to read local server path")?;
                    continue;
                }
                let local_server_path = local_server_path.trim().to_string();

                stdout.execute(PrintStyledContent("Where on localhost does this server run exactly?: ".white().bold()))?;
                stdout.flush()?;
                let mut server_path = String::new();
                if io::stdin().read_line(&mut server_path).is_err() {
                    print_error(&mut stdout, "Unable to read server path")?;
                    continue;
                }
                let server_path = server_path.trim().to_string();

                print_status(&mut stdout, &format!("Setting up NaomiiRouter on port {}...", port))?;

                match setup(port, server_path.clone(), local_server_path.clone()).await {
                    Ok(_) => {
                        print_success(&mut stdout, &format!(
                            "Router successfully started on port {} (path: {})",
                            port, server_path
                        ))?;
                    }
                    Err(_) => {
                        print_warning(&mut stdout, "Port unavailable, selecting a free port instead...")?;
                        let free_port = get_free_port();
                        match setup(free_port, server_path.clone(), local_server_path.clone()).await {
                            Ok(_) => {
                                print_success(&mut stdout, &format!(
                                    "Router started on port {} (path: {})",
                                    free_port, server_path
                                ))?;
                            }
                            Err(_) => {
                                print_error(&mut stdout, &format!(
                                    "Could not set up NaomiiRouter on port {} (path: {})",
                                    free_port, server_path
                                ))?;
                            }
                        }
                    }
                };
            }

            "!clear" | "clear" | "cls" => {
                stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                stdout.execute(cursor::MoveTo(0, 0))?;
                print_banner(&mut stdout)?;
            }

            "!status" | "status" => print_status_info(&mut stdout)?,

            "!exit" | "exit" | "!quit" | "quit" | "q" => {
                print_goodbye(&mut stdout)?;
                break;
            }

            _ => {
                print_error(&mut stdout, &format!("Unknown command: '{}'", cmd))?;
                stdout.execute(PrintStyledContent("Type ".grey()))?;
                stdout.execute(PrintStyledContent("help".yellow().bold()))?;
                stdout.execute(PrintStyledContent(" to see available commands.\n".grey()))?;
            }
        }
    }

    Ok(())
}

fn print_banner(stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("Naomii Startup Manager".cyan().bold()))?;
    Ok(())
}

fn print_help(stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("Available Commands\n".cyan().bold()))?;
    stdout.execute(PrintStyledContent("────────────────────────────────────────\n".dark_grey()))?;
    print_command(stdout, "!help", "Show this help menu")?;
    print_command(stdout, "!startup", "Start Naomii installation")?;
    print_command(stdout, "!setnew", "Set up server on custom port")?;
    print_command(stdout, "!status", "Show system status")?;
    print_command(stdout, "!clear", "Clear the screen")?;
    print_command(stdout, "!exit", "Exit the program")?;
    Ok(())
}

fn print_command(stdout: &mut io::Stdout, cmd: &str, desc: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("  ".white()))?;
    stdout.execute(PrintStyledContent(cmd.yellow().bold()))?;
    stdout.execute(PrintStyledContent(" — ".dark_grey()))?;
    stdout.execute(PrintStyledContent(format!("{}\n", desc).white()))?;
    Ok(())
}

fn print_section_header(stdout: &mut io::Stdout, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent(format!("{}\n", title).cyan().bold()))?;
    Ok(())
}

fn print_success(stdout: &mut io::Stdout, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent(format!("{}\n", msg).green().bold()))?;
    Ok(())
}

fn print_error(stdout: &mut io::Stdout, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("Error: ".red().bold()))?;
    stdout.execute(PrintStyledContent(format!("{}\n", msg).red()))?;
    Ok(())
}

fn print_warning(stdout: &mut io::Stdout, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent(format!("{}\n", msg).yellow().bold()))?;
    Ok(())
}

fn print_status(stdout: &mut io::Stdout, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent(format!("{}\n", msg).blue()))?;
    Ok(())
}

fn print_status_info(stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("System Status\n".cyan().bold()))?;
    stdout.execute(PrintStyledContent("────────────────────────────────────────\n".dark_grey()))?;
    stdout.execute(PrintStyledContent("Naomii Core: ".green().bold()))?;
    stdout.execute(PrintStyledContent("Running\n".white()))?;
    stdout.execute(PrintStyledContent("Router: ".yellow().bold()))?;
    stdout.execute(PrintStyledContent("Idle\n".white()))?;
    stdout.execute(PrintStyledContent("Services: ".blue().bold()))?;
    stdout.execute(PrintStyledContent("Ready\n".white()))?;
    Ok(())
}

fn print_goodbye(stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(PrintStyledContent("────────────────────────────────────────\n".dark_grey()))?;
    stdout.execute(PrintStyledContent("Thanks for using Naomii\n".cyan().bold()))?;
    stdout.execute(PrintStyledContent("Goodbye.\n".white()))?;
    Ok(())
}
