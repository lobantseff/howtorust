use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use howtorust::{get_chapter_examples, run_chapter_example, Difficulty, CHAPTERS};
use howtorust::ollama::OllamaClient;
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
                // Run specific example: howtorust <chapter> --example <name>
                run_specific_example(chapter_name, &args[3]);
            } else {
                // Show chapter: howtorust <chapter>
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
                        execute!(
                            stdout,
                            cursor::MoveLeft(1),
                            terminal::Clear(ClearType::UntilNewLine)
                        )?;
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
        println!("{}", "howtorust - Interactive Rust Tutorial".bold().cyan());
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
    println!("{}", "howtorust - Interactive Rust Tutorial".bold().cyan());
    println!();
    println!("Usage:");
    println!(
        "  {}                       Start interactive mode",
        "howtorust".green()
    );
    println!(
        "  {} <chapter>              Show examples for a chapter",
        "howtorust".green()
    );
    println!(
        "  {} --list                 List all available chapters",
        "howtorust".green()
    );
    println!(
        "  {} <chapter> --example <name>  Run a specific example",
        "howtorust".green()
    );
    println!(
        "  {} --help                 Show detailed help",
        "howtorust".green()
    );
    println!();
    println!("Examples:");
    println!(
        "  {}                      # Interactive mode",
        "howtorust".green()
    );
    println!(
        "  {} ownership             # View ownership chapter",
        "howtorust".green()
    );
    println!("  {} traits --example basic_trait", "howtorust".green());
    println!("  {} --list", "howtorust".green());
}

fn print_help() {
    println!("{}", "howtorust - Interactive Rust Tutorial".bold().cyan());
    println!();
    println!("{}", "DESCRIPTION:".bold());
    println!("  An interactive command-line tool for learning Rust concepts through");
    println!("  executable examples organized by topic.");
    println!();
    println!("{}", "USAGE:".bold());
    println!("  howtorust <chapter>                    Show and run examples for a chapter");
    println!("  howtorust --list                       List all available chapters");
    println!("  howtorust <chapter> --example <name>   Run a specific example");
    println!("  howtorust --help                       Show this help message");
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
    println!("  howtorust ownership              # Interactive ownership tutorial");
    println!("  howtorust traits                 # Learn about traits");
    println!("  howtorust closures --example move_keyword");
    println!();
    println!("{}", "WORKFLOW:".bold());
    println!("  1. List chapters: howtorust --list");
    println!("  2. Select a chapter: howtorust <chapter>");
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

    println!("{}", "Run 'howtorust <chapter>' to view examples.".dimmed());
}

fn show_chapter(chapter_name: &str, history: &mut Vec<String>) {
    let chapter = match howtorust::find_chapter_by_name(chapter_name) {
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

    println!(
        "{} ({} examples)",
        "Beginner".green().bold(),
        beginners.len()
    );
    for (_idx, example) in beginners.iter() {
        println!(
            "  {}. {} - {}",
            display_num,
            example.name.cyan(),
            example.description
        );
        display_num += 1;
    }
    println!();

    println!(
        "{} ({} examples)",
        "Intermediate".yellow().bold(),
        intermediates.len()
    );
    for (_idx, example) in intermediates.iter() {
        println!(
            "  {}. {} - {}",
            display_num,
            example.name.cyan(),
            example.description
        );
        display_num += 1;
    }
    println!();

    println!("{} ({} examples)", "Advanced".red().bold(), advanced.len());
    for (_idx, example) in advanced.iter() {
        println!(
            "  {}. {} - {}",
            display_num,
            example.name.cyan(),
            example.description
        );
        display_num += 1;
    }
    println!();

    // Interactive menu
    interactive_menu(chapter_name, &examples, history);
}

fn interactive_menu(
    chapter_name: &str,
    examples: &[howtorust::Example],
    history: &mut Vec<String>,
) {
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

fn display_and_run_example(chapter_name: &str, example: &howtorust::Example) {
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

    println!("{}", "Commentary:".bold().blue());
    println!("{}", "-".repeat(60).dimmed());
    for line in example.commentary.lines() {
        println!("{}", line);
    }
    println!("{}", "-".repeat(60).dimmed());
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

    // Show interactive menu
    example_actions_menu(chapter_name, example);
}

fn example_actions_menu(chapter_name: &str, example: &howtorust::Example) {
    let mut history = Vec::new();

    loop {
        println!();
        println!("{}", "Commands: /chat (or /c) - chat with AI | /run - run again | /code - show code | /back - return".dimmed());
        print!("{} ", ">".green().bold());
        io::stdout().flush().unwrap();

        let input = match read_line_with_history(&mut history) {
            Ok(line) => line,
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        };

        let cmd = input.trim();

        match cmd {
            "/chat" | "/c" => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    start_chat_mode(example).await;
                });
            }
            "/run" | "/r" => {
                println!();
                println!("{}", "Output:".bold().magenta());
                println!("{}", "-".repeat(60).dimmed());
                run_chapter_example(chapter_name, example.name);
                println!("{}", "-".repeat(60).dimmed());
            }
            "/code" => {
                println!();
                println!("{}", "Code:".bold().green());
                println!("{}", "-".repeat(60).dimmed());
                print_code_with_syntax_highlighting(example.code);
                println!("{}", "-".repeat(60).dimmed());
            }
            "/back" | "/b" | "quit" | "exit" | "" => {
                break;
            }
            _ => {
                println!("{} Unknown command: {}", "Error:".red().bold(), cmd);
            }
        }
    }
}

async fn start_chat_mode(example: &howtorust::Example) {
    println!();
    println!("{}", "=".repeat(60).cyan());
    println!("{}", "Chat Mode - Ask questions about this example".bold().cyan());
    println!("{}", "Type /quit, /exit, or /q to exit chat".dimmed());
    println!("{}", "=".repeat(60).cyan());
    println!();

    let ollama = OllamaClient::new(
        "http://localhost:11434".to_string(),
        "deepseek-v3.1:671b-cloud".to_string(),
    );

    // Initialize conversation with example context
    let mut messages: Vec<(String, String)> = vec![
        (
            "system".to_string(),
            format!(
                "You are a helpful Rust programming assistant. The user is learning about the following example:\n\n\
                Example: {}\n\
                Description: {}\n\n\
                Commentary:\n{}\n\n\
                Code:\n{}\n\n\
                Please help the user understand this example by answering their questions.",
                example.name,
                example.description,
                example.commentary,
                example.code
            ),
        ),
    ];

    let mut chat_history: Vec<String> = Vec::new();

    loop {
        print!("{} ", "You:".green().bold());
        io::stdout().flush().unwrap();

        let user_input = match read_chat_input(&mut chat_history) {
            Ok(input) => input,
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        };

        let trimmed = user_input.trim();

        // Check for exit commands
        if trimmed == "/quit" || trimmed == "/exit" || trimmed == "/q" {
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        // Add user message to conversation
        messages.push(("user".to_string(), trimmed.to_string()));

        // Get AI response
        print!("{} ", "AI:".cyan().bold());
        io::stdout().flush().unwrap();

        match ollama.chat_stream(&messages, |chunk| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        }).await {
            Ok(response) => {
                println!();

                // Word wrap the response for display (already printed during streaming)
                // Just add to message history
                messages.push(("assistant".to_string(), response));
            }
            Err(e) => {
                println!("{} Failed to get response: {}", "Error:".red().bold(), e);
                println!("{}", "Make sure Ollama is running at localhost:11434".yellow());
                println!();
                messages.pop(); // Remove the user message if we couldn't get a response
            }
        }
    }

    println!();
    println!("{}", "Exiting chat mode...".dimmed());
    println!();
}

fn read_chat_input(history: &mut Vec<String>) -> io::Result<String> {
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
                    return Ok(String::from("/quit"));
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
                        execute!(
                            stdout,
                            cursor::MoveLeft(1),
                            terminal::Clear(ClearType::UntilNewLine)
                        )?;
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
                            execute!(
                                stdout,
                                cursor::MoveToColumn(0),
                                terminal::Clear(ClearType::CurrentLine)
                            )?;
                            print!("{} ", "You:".green().bold());
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
                            print!("{} ", "You:".green().bold());
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
                            print!("{} ", "You:".green().bold());
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
        // Check if line is a comment
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            println!("{}", line.dimmed());
            continue;
        }

        // Enhanced syntax highlighting
        let mut output = String::new();
        let mut chars = line.chars().peekable();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_buffer = String::new();

        while let Some(ch) = chars.next() {
            // Handle string literals
            if ch == '"' {
                if in_string {
                    string_buffer.push(ch);
                    output.push_str(&string_buffer.bright_green().to_string());
                    string_buffer.clear();
                    in_string = false;
                } else {
                    if !current_word.is_empty() {
                        output.push_str(&colorize_word(&current_word));
                        current_word.clear();
                    }
                    string_buffer.push(ch);
                    in_string = true;
                }
                continue;
            }

            if in_string {
                string_buffer.push(ch);
                continue;
            }

            // Handle words and identifiers
            if ch.is_alphanumeric() || ch == '_' {
                current_word.push(ch);
            } else {
                if !current_word.is_empty() {
                    output.push_str(&colorize_word(&current_word));
                    current_word.clear();
                }

                // Check for special characters and operators
                if ch == '&' || ch == '*' || ch == '!' {
                    output.push_str(&ch.to_string().yellow().to_string());
                } else {
                    output.push(ch);
                }
            }
        }

        // Handle remaining word
        if !current_word.is_empty() {
            output.push_str(&colorize_word(&current_word));
        }

        // Handle unterminated string
        if in_string {
            output.push_str(&string_buffer.bright_green().to_string());
        }

        println!("{}", output);
    }
}

fn colorize_word(word: &str) -> String {
    match word {
        // Keywords - Blue
        "fn" | "pub" | "struct" | "enum" | "impl" | "trait" | "async" | "await" | "type"
        | "where" | "unsafe" | "extern" | "dyn" => word.blue().to_string(),
        // Variable declarations - Green
        "let" | "mut" | "const" | "static" | "ref" => word.green().to_string(),
        // Module system - Magenta
        "use" | "mod" | "crate" | "self" | "super" | "as" | "in" => word.magenta().to_string(),
        // Control flow - Yellow
        "if" | "else" | "match" | "loop" | "while" | "for" | "return" | "break" | "continue"
        | "yield" => word.yellow().to_string(),
        // Boolean and special literals - Cyan
        "true" | "false" | "None" | "Some" | "Ok" | "Err" => word.cyan().to_string(),
        // Common types - Bright Blue
        "String" | "Vec" | "Option" | "Result" | "Box" | "Rc" | "Arc" | "HashMap" | "HashSet"
        | "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "f32"
        | "f64" | "bool" | "char" | "str" | "usize" | "isize" => word.bright_blue().to_string(),
        // Move semantics - Bright Yellow
        "move" | "Copy" | "Clone" | "Drop" => word.bright_yellow().to_string(),
        _ => {
            // Check if it's a number
            if word.chars().all(|c| c.is_numeric() || c == '.' || c == '_') {
                word.bright_magenta().to_string()
            } else {
                word.to_string()
            }
        }
    }
}
