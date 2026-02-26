use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: MessageResponse,
    #[allow(dead_code)]
    done: bool,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: String,
}

pub struct OllamaClient {
    base_url: String,
    model: String,
}

#[allow(dead_code)]
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.next() == Some('[') {
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn format_rust_char(ch: char) -> String {
    // Simple Rust syntax highlighting
    ch.to_string().bright_yellow().to_string()
}

fn format_markdown_char(ch: char, state: &mut MarkdownState) -> Option<String> {
    // Track backticks for code blocks
    if ch == '`' {
        if state.at_line_start || state.code_block_backticks > 0 {
            state.code_block_backticks += 1;
            state.last_char = Some(ch);

            // Check if we have 3 backticks (code block delimiter)
            if state.code_block_backticks == 3 {
                state.in_code_block = !state.in_code_block;
                state.code_block_backticks = 0;

                // If exiting code block, reset rust flag
                if !state.in_code_block {
                    state.in_rust_block = false;
                }
                return None; // Hide the backticks
            }
            return None;
        } else {
            // Single backtick inline code
            state.in_code = !state.in_code;
            state.last_char = Some(ch);
            return None;
        }
    } else if state.code_block_backticks > 0 && state.code_block_backticks < 3 {
        // We had some backticks but not 3, output them and continue
        let backticks = "`".repeat(state.code_block_backticks);
        state.code_block_backticks = 0;
        let mut result = backticks;
        result.push(ch);
        state.last_char = Some(ch);
        state.at_line_start = false;
        return Some(result);
    }

    // Check if we just entered a code block and the next chars are "rust"
    if state.in_code_block && !state.in_rust_block && state.at_line_start {
        // Buffer to check for "rust"
        if ch == 'r' || ch == 'u' || ch == 's' || ch == 't' {
            state.last_char = Some(ch);
            return None; // Hide language identifier
        } else if ch == '\n' && state.last_char == Some('t') {
            // We just saw "rust" and now a newline
            state.in_rust_block = true;
            state.at_line_start = true;
            state.last_char = Some(ch);
            return Some("\n".to_string());
        } else if ch == '\n' {
            // Just a newline after opening backticks, assume rust
            state.in_rust_block = true;
            state.at_line_start = true;
            state.last_char = Some(ch);
            return Some("\n".to_string());
        }
    }

    // If we're in a rust code block, apply syntax highlighting
    if state.in_rust_block && ch != '\n' {
        let formatted = format_rust_char(ch);
        state.last_char = Some(ch);
        state.at_line_start = false;
        return Some(formatted);
    }

    // Track consecutive dashes for horizontal rules
    if ch == '-' && state.at_line_start && !state.in_code && !state.in_code_block {
        state.consecutive_dashes += 1;
        state.last_char = Some(ch);
        return None;
    } else if state.consecutive_dashes >= 3 && (ch == '\n' || ch == ' ') {
        // Found horizontal rule (--- or more)
        state.consecutive_dashes = 0;
        if ch == '\n' {
            state.at_line_start = true;
            return Some(
                "────────────────────────────────────────────────────────────\n"
                    .dimmed()
                    .to_string(),
            );
        } else {
            return None; // Skip spaces after ---
        }
    } else if state.consecutive_dashes == 1 && ch == ' ' {
        // Single dash followed by space = bullet point, just output "- "
        state.consecutive_dashes = 0;
        state.at_line_start = false;
        state.last_char = Some(ch);
        return Some("- ".to_string());
    } else if state.consecutive_dashes > 0 && state.consecutive_dashes < 3 && ch != '-' && ch != ' '
    {
        // Not enough dashes for horizontal rule, and not a bullet (no space after)
        // Output the dashes and continue processing current char
        let mut result = "-".repeat(state.consecutive_dashes);
        state.consecutive_dashes = 0;
        state.at_line_start = false;

        // Process current character and append it
        let formatted = if state.in_code {
            ch.to_string().bright_yellow().to_string()
        } else if state.in_bold {
            ch.to_string().bold().to_string()
        } else {
            ch.to_string()
        };
        result.push_str(&formatted);
        state.last_char = Some(ch);
        return Some(result);
    }

    match ch {
        '*' if state.last_char == Some('*') && !state.in_code => {
            // Toggle bold mode on **
            state.in_bold = !state.in_bold;
            state.last_char = None; // Reset to avoid triple *
            None
        }
        '*' if !state.in_code => {
            // Might be start/end of bold, wait for next char
            state.last_char = Some(ch);
            None
        }
        '#' if state.at_line_start && !state.in_code => {
            state.in_heading = true;
            state.at_line_start = false;
            state.last_char = Some(ch);
            None
        }
        ' ' if state.in_heading && state.heading_level == 0 => {
            state.heading_level = state.heading_hashes;
            state.heading_hashes = 0;
            state.last_char = Some(ch);
            None
        }
        '\n' => {
            state.at_line_start = true;
            state.in_heading = false;
            state.heading_level = 0;
            state.heading_hashes = 0;
            state.consecutive_dashes = 0;
            state.last_char = Some(ch);
            Some("\n".to_string())
        }
        _ => {
            // If last char was a single *, output it now
            let mut result = String::new();
            if state.last_char == Some('*') {
                result.push('*');
            }
            state.last_char = Some(ch);
            state.at_line_start = false;

            if state.in_heading && ch == '#' && state.heading_level == 0 {
                state.heading_hashes += 1;
                None
            } else {
                let formatted = if state.in_code {
                    ch.to_string().bright_yellow().to_string()
                } else if state.in_heading {
                    ch.to_string().bright_cyan().bold().to_string()
                } else if state.in_bold {
                    ch.to_string().bold().to_string()
                } else {
                    ch.to_string()
                };
                result.push_str(&formatted);
                Some(result)
            }
        }
    }
}

struct MarkdownState {
    in_code: bool,
    in_heading: bool,
    heading_level: usize,
    heading_hashes: usize,
    at_line_start: bool,
    in_bold: bool,
    last_char: Option<char>,
    consecutive_dashes: usize,
    in_code_block: bool,
    code_block_backticks: usize,
    in_rust_block: bool,
}

impl MarkdownState {
    fn new() -> Self {
        Self {
            in_code: false,
            in_heading: false,
            heading_level: 0,
            heading_hashes: 0,
            at_line_start: true,
            in_bold: false,
            last_char: None,
            consecutive_dashes: 0,
            in_code_block: false,
            code_block_backticks: 0,
            in_rust_block: false,
        }
    }
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self { base_url, model }
    }

    pub async fn chat_stream<F>(
        &self,
        messages: &[(String, String)],
        mut callback: F,
    ) -> Result<String, Box<dyn Error>>
    where
        F: FnMut(&str),
    {
        let url = format!("{}/api/chat", self.base_url);

        let formatted_messages: Vec<Message> = messages
            .iter()
            .map(|(role, content)| Message {
                role: role.clone(),
                content: content.clone(),
            })
            .collect();

        let request = ChatRequest {
            model: self.model.clone(),
            messages: formatted_messages,
            stream: true,
        };

        let client = reqwest::Client::builder()
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .build()?;
        let mut response = client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(format!("Ollama API error: {}", response.status()).into());
        }

        let mut full_response = String::new();
        let mut current_line_visible_len: usize = 0;
        let mut current_line_buf = String::new();
        let max_width = 60;
        let mut buffer = String::new();
        let mut md_state = MarkdownState::new();

        // Stream the response using chunk() for true incremental delivery
        while let Some(chunk_bytes) = response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk_bytes);
            buffer.push_str(&chunk_str);

            // Process complete lines from the buffer
            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if line.trim().is_empty() {
                    continue;
                }

                if let Ok(chat_response) = serde_json::from_str::<ChatResponse>(&line) {
                    let content = &chat_response.message.content;
                    full_response.push_str(content);

                    // Process each character with markdown formatting and emit immediately
                    for ch in content.chars() {
                        if let Some(formatted) = format_markdown_char(ch, &mut md_state) {
                            if ch == '\n' {
                                callback("\n");
                                current_line_visible_len = 0;
                                current_line_buf.clear();
                            } else if ch.is_whitespace()
                                && !md_state.in_code
                                && !md_state.in_code_block
                            {
                                if current_line_visible_len >= max_width {
                                    // Wrap at whitespace boundary
                                    callback("\n");
                                    current_line_visible_len = 0;
                                    current_line_buf.clear();
                                } else {
                                    callback(&formatted);
                                    current_line_visible_len += 1;
                                    current_line_buf.push(ch);
                                }
                            } else {
                                callback(&formatted);
                                current_line_visible_len += 1;
                                current_line_buf.push(ch);

                                // Force wrap very long words (skip in code blocks)
                                if !md_state.in_code
                                    && !md_state.in_code_block
                                    && current_line_visible_len > max_width + 10
                                {
                                    callback("\n");
                                    current_line_visible_len = 0;
                                    current_line_buf.clear();
                                }
                            }
                            // Small delay per visible character to ensure
                            // streaming is perceptible in the terminal
                            tokio::time::sleep(Duration::from_millis(2)).await;
                        }
                    }
                }
            }
        }

        Ok(full_response)
    }
}
