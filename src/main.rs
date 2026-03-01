use clap::{Parser, Subcommand};
use comfy_table::{Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};
use owo_colors::OwoColorize;
use std::io::{self, Write};
mod generated {
    include!(concat!(env!("OUT_DIR"), "/problems_gen.rs"));
}

#[derive(Parser, Debug)]
#[command(
    name = "problem-runner",
    version,
    about = "List and run problems from the problems/ folder"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Run { number: usize },
}

fn print_banner() {
    println!("{}", "Euler Project".bold().fg_rgb::<16, 124, 255>());
}

fn print_list() {
    print_banner();

    if generated::PROBLEMS.is_empty() {
        println!("{}", "No .rs files found in problems/".yellow().bold());
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Problem").fg(Color::Green),
            Cell::new("Entry").fg(Color::Yellow),
        ]);

    for p in generated::PROBLEMS {
        table.add_row(vec![
            Cell::new(p.name),
            Cell::new("solve()").fg(Color::Yellow),
        ]);
    }

    println!("{table}");
    println!(
        "\n{} {}",
        "Run example:".bold(),
        "cargo run -- run 1".cyan()
    );
    if let Some(k) = read_usize_pretty() {
        run_problem(k);
    }
}
fn read_usize_pretty() -> Option<usize> {
    println!("{}", "Enter 0 to exit.\n".dimmed());

    loop {
        print!("{} ", "Enter problem number:".cyan().bold());
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let trimmed = input.trim();

        match trimmed.parse::<usize>() {
            Ok(0) => return None,
            Ok(n) => {
                println!("\n");
                return Some(n);
            }
            Err(_) => {
                let mut t = Table::new();
                t.load_preset(UTF8_FULL).set_header(vec![
                    Cell::new("Status").fg(Color::Red),
                    Cell::new("Message").fg(Color::Yellow),
                ]);
                t.add_row(vec![
                    Cell::new("Invalid"),
                    Cell::new(format!("'{}' is not a valid usize. Try again.", trimmed)),
                ]);
                println!("\n{t}\n");
            }
        }
    }
}

fn run_problem(number: usize) {
    if number == 0 {
        eprintln!("{}", "Please provide a positive number (1, 2, 3...)".red());
        std::process::exit(2);
    }

    match generated::PROBLEMS
        .iter()
        .find(|p| p.name == number.to_string())
    {
        Some(problem) => {
            println!(
                "{} {}",
                "Running".green().bold(),
                problem.name.magenta().bold()
            );
            println!("{}", "- ".repeat(10).dimmed());
            (problem.solve)();
        }
        None => {
            eprintln!(
                "{} {}",
                "Problem not found for number:".red().bold(),
                number.to_string().cyan()
            );
            eprintln!(
                "Use {} to see valid indexes.",
                "`cargo run -- list`".yellow()
            );
            std::process::exit(2);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::List) => print_list(),
        Some(Commands::Run { number }) => run_problem(number),
    }
}

