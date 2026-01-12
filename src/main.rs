use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use howrust::{get_chapter_examples, run_chapter_example, Difficulty, CHAPTERS};
use std::env;
use std::io::{self, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        interactive_mode();
        return;
    }

    match args[1].as_str() {
        "--list" | "-l" => list_chapters(),
        "--help" | "-h" => print_help(),
        chapter_name => {
            if args.len() >= 4 && args[2] == "--example" {
                // Run specific example: howrust <chapter> --example <name>
                run_specific_example(chapter_name, &args[3]);
            } else {
                // Show chapter: howrust <chapter>
                let mut history = Vec::new();
                show_chapter(chapter_name, &mut history);
            }
        }
    }
}

fn read_line_with_history(history: &mut Vec<String>) -> io::Result<String> {
    let mut input = String::new();
    let mut history_index: Option<usize> = None;
    let mut current_input = String::new();

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    loop {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match code {
                KeyCode::Enter => {
                    terminal::disable_raw_mode()?;
                    println!();
                    if !input.trim().is_empty() && !history.contains(&input.trim().to_string()) {
                        history.push(input.trim().to_string());
                    }
                    return Ok(input);
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode()?;
                    println!();
                    return Ok(String::from("quit"));
                }
                KeyCode::Char(c) => {
                    input.insert(input.len(), c);
                    print!("{}", c);
                    stdout.flush()?;
                    current_input = input.clone();
                    history_index = None;
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        execute!(stdout, cursor::MoveLeft(1), terminal::Clear(ClearType::UntilNewLine))?;
                        current_input = input.clone();
                        history_index = None;
                    }
                }
                KeyCode::Up => {
                    if !history.is_empty() {
                        if history_index.is_none() {
                            current_input = input.clone();
                            history_index = Some(history.len() - 1);
                        } else if let Some(idx) = history_index {
                            if idx > 0 {
                                history_index = Some(idx - 1);
                            }
                        }

                        if let Some(idx) = history_index {
                            // Clear current line
                            execute!(
                                stdout,
                                cursor::MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )?;
                            print!("{} ", "Choose:".green().bold());
                            input = history[idx].clone();
                            print!("{}", input);
                            stdout.flush()?;
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(idx) = history_index {
                        if idx < history.len() - 1 {
                            history_index = Some(idx + 1);
                            execute!(
                                stdout,
                                cursor::MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )?;
                            print!("{} ", "Choose:".green().bold());
                            input = history[history_index.unwrap()].clone();
                            print!("{}", input);
                            stdout.flush()?;
                        } else {
                            history_index = None;
                            execute!(
                                stdout,
                                cursor::MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )?;
                            print!("{} ", "Choose:".green().bold());
                            input = current_input.clone();
                            print!("{}", input);
                            stdout.flush()?;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn interactive_mode() {
    let mut history: Vec<String> = Vec::new();

    loop {
        println!();
        println!("{}", "HowRust - Interactive Rust Tutorial".bold().cyan());
        println!("{}", "=".repeat(40).cyan());
        println!();

        for (idx, chapter) in CHAPTERS.iter().enumerate() {
            println!(
                "{}. {} - {}",
                (idx + 1).to_string().yellow(),
                chapter.name.green().bold(),
                chapter.title
            );
        }

        println!();
        println!("{}", "Options:".bold());
        println!("  [number] - Select a chapter");
        println!("  {} - Show help", "help".cyan());
        println!("  {} - Quit", "quit".cyan());
        print!("\n{} ", "Choose:".green().bold());
        io::stdout().flush().unwrap();

        let input = match read_line_with_history(&mut history) {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        };
        let input = input.trim();

        match input {
            "quit" | "q" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" | "h" => {
                print_help();
            }
            _ => {
                if let Ok(num) = input.parse::<usize>() {
                    if num > 0 && num <= CHAPTERS.len() {
                        let chapter = &CHAPTERS[num - 1];
                        show_chapter(chapter.name, &mut history);
                    } else {
                        println!(
                            "{} Please enter a number between 1 and {}",
                            "Error:".red(),
                            CHAPTERS.len()
                        );
                    }
                } else {
                    println!("{} Invalid input", "Error:".red());
                }
            }
        }
    }
}

#[allow(dead_code)]
fn print_usage() {
    println!("{}", "HowRust - Interactive Rust Tutorial".bold().cyan());
    println!();
    println!("Usage:");
    println!("  {}                       Start interactive mode", "howrust".green());
    println!("  {} <chapter>              Show examples for a chapter", "howrust".green());
    println!(
        "  {} --list                 List all available chapters",
        "howrust".green()
    );
    println!(
        "  {} <chapter> --example <name>  Run a specific example",
        "howrust".green()
    );
    println!("  {} --help                 Show detailed help", "howrust".green());
    println!();
    println!("Examples:");
    println!("  {}                      # Interactive mode", "howrust".green());
    println!("  {} ownership             # View ownership chapter", "howrust".green());
    println!("  {} traits --example basic_trait", "howrust".green());
    println!("  {} --list", "howrust".green());
}

fn print_help() {
    println!("{}", "HowRust - Interactive Rust Tutorial".bold().cyan());
    println!();
    println!("{}", "DESCRIPTION:".bold());
    println!("  An interactive command-line tool for learning Rust concepts through");
    println!("  executable examples organized by topic.");
    println!();
    println!("{}", "USAGE:".bold());
    println!("  howrust <chapter>                    Show and run examples for a chapter");
    println!("  howrust --list                       List all available chapters");
    println!("  howrust <chapter> --example <name>   Run a specific example");
    println!("  howrust --help                       Show this help message");
    println!();
    println!("{}", "AVAILABLE CHAPTERS:".bold());
    for chapter in CHAPTERS {
        println!(
            "  {} - {}",
            chapter.name.green(),
            chapter.description.dimmed()
        );
    }
    println!();
    println!("{}", "EXAMPLES:".bold());
    println!("  howrust ownership              # Interactive ownership tutorial");
    println!("  howrust traits                 # Learn about traits");
    println!("  howrust closures --example move_keyword");
    println!();
    println!("{}", "WORKFLOW:".bold());
    println!("  1. List chapters: howrust --list");
    println!("  2. Select a chapter: howrust <chapter>");
    println!("  3. View and run examples interactively");
}

fn list_chapters() {
    println!("{}", "Available Chapters:".bold().cyan());
    println!();

    for (idx, chapter) in CHAPTERS.iter().enumerate() {
        println!(
            "{}. {} - {}",
            (idx + 1).to_string().yellow(),
            chapter.name.green().bold(),
            chapter.title
        );
        println!("   {}", chapter.description.dimmed());
        println!();
    }

    println!(
        "{}",
        "Run 'howrust <chapter>' to view examples.".dimmed()
    );
}

fn show_chapter(chapter_name: &str, history: &mut Vec<String>) {
    let chapter = match howrust::find_chapter_by_name(chapter_name) {
        Some(ch) => ch,
        None => {
            println!(
                "{} Chapter '{}' not found",
                "Error:".red().bold(),
                chapter_name.yellow()
            );
            println!();
            println!("Available chapters:");
            for ch in CHAPTERS {
                println!("  - {}", ch.name.green());
            }
            return;
        }
    };

    let examples = match get_chapter_examples(chapter_name) {
        Some(ex) => ex,
        None => {
            println!("No examples found for chapter '{}'", chapter_name);
            return;
        }
    };

    println!();
    println!("{}", chapter.title.bold().cyan());
    println!("{}", "=".repeat(chapter.title.len()).cyan());
    println!("{}", chapter.description.dimmed());
    println!();

    // Group examples by difficulty
    let beginners: Vec<_> = examples
        .iter()
        .enumerate()
        .filter(|(_, e)| e.difficulty == Difficulty::Beginner)
        .collect();
    let intermediates: Vec<_> = examples
        .iter()
        .enumerate()
        .filter(|(_, e)| e.difficulty == Difficulty::Intermediate)
        .collect();
    let advanced: Vec<_> = examples
        .iter()
        .enumerate()
        .filter(|(_, e)| e.difficulty == Difficulty::Advanced)
        .collect();

    let mut display_num = 1;

    println!("{} ({} examples)", "Beginner".green().bold(), beginners.len());
    for (_idx, example) in beginners.iter() {
        println!("  {}. {} - {}", display_num, example.name.cyan(), example.description);
        display_num += 1;
    }
    println!();

    println!(
        "{} ({} examples)",
        "Intermediate".yellow().bold(),
        intermediates.len()
    );
    for (_idx, example) in intermediates.iter() {
        println!("  {}. {} - {}", display_num, example.name.cyan(), example.description);
        display_num += 1;
    }
    println!();

    println!(
        "{} ({} examples)",
        "Advanced".red().bold(),
        advanced.len()
    );
    for (_idx, example) in advanced.iter() {
        println!("  {}. {} - {}", display_num, example.name.cyan(), example.description);
        display_num += 1;
    }
    println!();

    // Interactive menu
    interactive_menu(chapter_name, &examples, history);
}

fn interactive_menu(chapter_name: &str, examples: &[howrust::Example], history: &mut Vec<String>) {
    loop {
        println!("{}", "Options:".bold());
        println!("  [number] - View and run an example");
        println!("  {} - List all examples", "list".cyan());
        println!("  {} - Back to chapters menu", "back".cyan());
        print!("\n{} ", "Choose:".green().bold());
        io::stdout().flush().unwrap();

        let input = match read_line_with_history(history) {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        };
        let input = input.trim();

        match input {
            "quit" | "q" | "exit" | "back" | "b" => {
                break;
            }
            "list" | "l" => {
                println!();
                for (idx, example) in examples.iter().enumerate() {
                    println!(
                        "  {}. {} ({}) - {}",
                        idx + 1,
                        example.name.cyan(),
                        format!("{:?}", example.difficulty).dimmed(),
                        example.description
                    );
                }
                println!();
            }
            _ => {
                if let Ok(num) = input.parse::<usize>() {
                    if num > 0 && num <= examples.len() {
                        display_and_run_example(chapter_name, &examples[num - 1]);
                    } else {
                        println!(
                            "{} Please enter a number between 1 and {}",
                            "Error:".red(),
                            examples.len()
                        );
                    }
                } else {
                    println!("{} Invalid input", "Error:".red());
                }
                println!();
            }
        }
    }
}

fn display_and_run_example(chapter_name: &str, example: &howrust::Example) {
    println!();
    println!("{}", "=".repeat(60).cyan());
    println!(
        "{} {}",
        "Example:".bold().cyan(),
        example.name.bold().white()
    );
    println!(
        "{} {}",
        "Level:".bold(),
        format!("{:?}", example.difficulty).yellow()
    );
    println!("{} {}", "Description:".bold(), example.description);
    println!("{}", "=".repeat(60).cyan());
    println!();

    println!("{}", "Code:".bold().green());
    println!("{}", "-".repeat(60).dimmed());
    print_code_with_syntax_highlighting(example.code);
    println!("{}", "-".repeat(60).dimmed());
    println!();

    println!("{}", "Output:".bold().magenta());
    println!("{}", "-".repeat(60).dimmed());
    run_chapter_example(chapter_name, example.name);
    println!("{}", "-".repeat(60).dimmed());
    println!();

    println!("{}", "Commentary:".bold().blue());
    println!("{}", "-".repeat(60).dimmed());
    for line in example.commentary.lines() {
        println!("{}", line);
    }
    println!("{}", "-".repeat(60).dimmed());
    println!();
}

fn run_specific_example(chapter_name: &str, example_name: &str) {
    let examples = match get_chapter_examples(chapter_name) {
        Some(ex) => ex,
        None => {
            println!("Chapter '{}' not found", chapter_name);
            return;
        }
    };

    let example = examples.iter().find(|e| e.name == example_name);

    match example {
        Some(ex) => display_and_run_example(chapter_name, ex),
        None => {
            println!(
                "{} Example '{}' not found in chapter '{}'",
                "Error:".red().bold(),
                example_name.yellow(),
                chapter_name.yellow()
            );
            println!();
            println!("Available examples:");
            for ex in examples {
                println!("  - {}", ex.name.cyan());
            }
        }
    }
}

fn print_code_with_syntax_highlighting(code: &str) {
    for line in code.lines() {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];

        // Simple syntax highlighting
        let highlighted = if trimmed.starts_with("//") {
            trimmed.dimmed().to_string()
        } else if trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("trait ")
        {
            trimmed.blue().bold().to_string()
        } else if trimmed.starts_with("let ") || trimmed.starts_with("const ") {
            trimmed.green().to_string()
        } else if trimmed.starts_with("use ") || trimmed.starts_with("mod ") {
            trimmed.magenta().to_string()
        } else {
            trimmed.to_string()
        };

        println!("{}{}", indent, highlighted);
    }
}
