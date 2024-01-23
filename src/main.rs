use std::io::Write;

use clap::{Parser, Subcommand, Args};

const PARSER_TEMPLATE: &str = "\
        {all-args}
";

const APPLET_TEMPLATE: &str = "\
    {about-with-newline}\n\
    {usage-heading}\n    {usage}\n\
    \n\
    {all-args}{after-help}\
";

#[derive(Debug, Parser)]
#[command(multicall = true, help_template = PARSER_TEMPLATE)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(help_template = APPLET_TEMPLATE)]
    Echo(EchoArgs),
    #[command(help_template = APPLET_TEMPLATE)]
    Ping,
    #[command(help_template = APPLET_TEMPLATE)]
    Quit,
}

#[derive(Args, Debug)]
pub struct EchoArgs {
    #[arg(
        short = 't', 
        long = "text", 
        visible_alias = "text",
        help = "The text to be echoed",
        help_heading = "Echo"
    )]
	text: String,
}

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Echo(args) => {
            write!(std::io::stdout(), "{}\n", args.text).map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Ping => {
            write!(std::io::stdout(), "Pong\n").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Quit => {
            write!(std::io::stdout(), "Exiting ...\n").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }

    }
    Ok(false)
}
fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}