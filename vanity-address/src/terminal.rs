use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

static PEACE_OUT_SHOWN: AtomicBool = AtomicBool::new(false);

/// Friendly farewell shown on exit or Ctrl+C.
pub fn peace_out() {
    if PEACE_OUT_SHOWN.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = disable_raw_mode();
    println!();
    println!(
        "  {}",
        "Peace out! vanity-address is closing.".cyan().bold()
    );
    println!(
        "  {}",
        "Stay safe with your keys — see you on the next grind.".dimmed()
    );
    println!();
    let _ = io::stdout().flush();
}

pub fn peace_out_and_exit() -> ! {
    peace_out();
    std::process::exit(0);
}

/// Catch Ctrl+C during grinding or outside raw-mode menus.
pub fn install_ctrlc_handler() {
    let _ = ctrlc::set_handler(|| {
        peace_out_and_exit();
    });
}

fn handle_ctrl_c() {
    disable_raw_mode().ok();
    peace_out_and_exit();
}

/// Menu selection: a digit in range, or back (0 / Esc).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuChoice {
    Back,
    Selected(u32),
}

/// Read a single key — digit selects, `0` or Esc goes back (if allowed).
pub fn read_menu_choice(prompt: &str, min: u32, max: u32, allow_back: bool) -> MenuChoice {
    print!("{prompt}");
    let _ = io::stdout().flush();

    enable_raw_mode().expect("terminal raw mode");
    let choice = loop {
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                handle_ctrl_c();
            }

            if allow_back && matches!(code, KeyCode::Esc | KeyCode::Char('0')) {
                let _ = writeln_str("← back");
                break MenuChoice::Back;
            }

            if let KeyCode::Char(c) = code {
                if let Some(digit) = c.to_digit(10) {
                    if (min..=max).contains(&digit) {
                        let _ = writeln_char(c);
                        break MenuChoice::Selected(digit);
                    }
                }
            }
        }
    };
    disable_raw_mode().expect("terminal restore");
    choice
}

/// Read y/n — `y` confirms, `n`/Esc/Enter declines. Esc on confirm screen = back.
pub fn read_yes_no_key(prompt: &str, esc_is_back: bool) -> Option<bool> {
    print!("{prompt}");
    let _ = io::stdout().flush();

    enable_raw_mode().expect("terminal raw mode");
    let result = loop {
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                handle_ctrl_c();
            }

            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    let _ = writeln_str("y");
                    break Some(true);
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    let _ = writeln_str("n");
                    break Some(false);
                }
                KeyCode::Enter => {
                    println!();
                    break Some(false);
                }
                KeyCode::Esc if esc_is_back => {
                    let _ = writeln_str("← back");
                    break None;
                }
                KeyCode::Esc => {
                    let _ = writeln_str("n");
                    break Some(false);
                }
                _ => {}
            }
        }
    };
    disable_raw_mode().expect("terminal restore");
    result
}

/// Text input with Enter to confirm, Esc to go back.
pub fn read_line_with_escape(prompt: &str) -> Option<String> {
    print!("{prompt}");
    let _ = io::stdout().flush();

    enable_raw_mode().expect("terminal raw mode");
    let mut buf = String::new();
    let result = loop {
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                handle_ctrl_c();
            }

            match code {
                KeyCode::Esc => {
                    println!();
                    break None;
                }
                KeyCode::Enter => {
                    println!();
                    break Some(buf.trim().to_string());
                }
                KeyCode::Backspace | KeyCode::Delete => {
                    buf.pop();
                    // erase char on screen
                    print!("\x08 \x08");
                    let _ = io::stdout().flush();
                }
                KeyCode::Char(c) => {
                    buf.push(c);
                    print!("{c}");
                    let _ = io::stdout().flush();
                }
                _ => {}
            }
        }
    };
    disable_raw_mode().expect("terminal restore");
    result
}

pub fn wait_for_key(message: &str) {
    print!("{message}");
    let _ = io::stdout().flush();
    enable_raw_mode().expect("terminal raw mode");
    loop {
        if let Ok(Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                handle_ctrl_c();
            }
            break;
        }
    }
    disable_raw_mode().expect("terminal restore");
    println!();
}

fn writeln_char(c: char) -> io::Result<()> {
    println!("{c}");
    io::stdout().flush()
}

fn writeln_str(s: &str) -> io::Result<()> {
    println!("{s}");
    io::stdout().flush()
}
