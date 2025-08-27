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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    // Clear screen and move to top
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    // Banner
    print_banner(&mut stdout)?;
    
    // Welcome message
    stdout.execute(PrintStyledContent("\nCommands start with ".grey()))?;
    stdout.execute(PrintStyledContent("!".yellow().bold()))?;
    stdout.execute(PrintStyledContent(" — type ".grey()))?;
    stdout.execute(PrintStyledContent("help".yellow().bold()))?;
    stdout.execute(PrintStyledContent(" to see available commands.\n\n".grey()))?;

    loop {
        // Prompt
        stdout.execute(PrintStyledContent(">> ".magenta().bold()))?;
        stdout.flush()?;

        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            print_error(&mut stdout, "Could not read command")?;
            continue;
        }

        let cmd = cmd.trim();

        match cmd {
            "!help" | "help" | "?" => {
                print_help(&mut stdout)?;
            }

            "!startup" | "startup" | "start" => {
                stdout.execute(PrintStyledContent("Starting Naomii installation...\n".cyan().bold()))?;
                todo!()
            }

            "!setnew" | "set" | "new" | "server" => {
                print_section_header(&mut stdout, "Server Setup")?;

                // Ask for port
                stdout.execute(PrintStyledContent("Enter a port for ".white()))?;
                stdout.execute(PrintStyledContent("NaomiiRouter".cyan().bold()))?;
                stdout.execute(PrintStyledContent(": ".white()))?;
                stdout.flush()?;            

                let mut custom_port = String::new();
                if io::stdin().read_line(&mut custom_port).is_err() {
                    print_error(&mut stdout, "Unable to read port")?;
                    continue;
                }
                stdout.execute(PrintStyledContent("Where does the bin for this server live?: ".blue().bold()))?;
                let mut local_server_path = String::new(); // where the server lives on the computer
                if io::stdin().read_line(&mut local_server_path).is_err() {
                    print_error(&mut stdout, "Unable to read local server path")?;
                    continue;
                }
                // Ask for server path
                stdout.execute(PrintStyledContent("Where on localhost does this server run exactly?: ".white()))?;
                stdout.flush()?;
                let mut server_path = String::new(); // very similar name but this is where the server runs (on localhost)
                if io::stdin().read_line(&mut server_path).is_err() {
                    print_error(&mut stdout, "Unable to read server path")?;
                    continue;
                }
                let server_path = server_path.trim().to_string(); // remove newline
                let local_server_path = local_server_path.trim().to_string();
                // Validate port
                let port: u32 = match custom_port.trim().parse::<u32>() {
                    Ok(num) if num > 0 && num < 65536 => num,
                    _ => {
                        print_error(&mut stdout, "Invalid port number (1-65535)")?;
                        continue;
                    }
                };

                print_status(&mut stdout, &format!("Setting up NaomiiRouter on port {}...", port))?;

                // Attempt setup
                match setup(port, server_path.clone(), local_server_path.clone()).await {
                    Ok(_) => {
                        print_success(&mut stdout, &format!(
                            "Router successfully started on port {} (path: {})",
                            port, server_path
                        ))?;
                    }
                    Err(_) => {
                        print_warning(&mut stdout, "Port unavailable, selecting a free port instead...")?;
                        let free_port = get_free_port(); // custom function, not the same as the one from port_check
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

            "!status" | "status" => {
                print_status_info(&mut stdout)?;
            }

            "!exit" | "exit" | "!quit" | "quit" | "q" => {
                print_goodbye(&mut stdout)?;
                break;
            }

            "" => continue, 

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
