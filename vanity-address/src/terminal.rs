use colored::Colorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

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

/// How long to wait for the next digit when `1` could mean option 1 or 10–13.
const MULTI_DIGIT_WAIT_MS: u64 = 2_000;

/// Whether another digit could still produce a valid selection ≤ max.
fn can_extend(value: u32, max: u32) -> bool {
    value > 0 && value.saturating_mul(10) <= max
}

fn is_press(kind: KeyEventKind) -> bool {
    // Older terminals may not report kind; treat anything that isn't Release/Repeat as press.
    !matches!(kind, KeyEventKind::Release | KeyEventKind::Repeat)
}

/// Read a digit selection. Unambiguous digits (e.g. 1–3 when max=3) select immediately.
/// Ambiguous prefixes (e.g. `1` when max=13) wait briefly for a second digit or Enter.
pub fn read_menu_choice(prompt: &str, min: u32, max: u32, allow_back: bool) -> MenuChoice {
    print!("{prompt}");
    let _ = io::stdout().flush();

    enable_raw_mode().expect("terminal raw mode");
    let choice = loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        })) = event::read()
        {
            if !is_press(kind) {
                continue;
            }

            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                handle_ctrl_c();
            }

            if allow_back && matches!(code, KeyCode::Esc | KeyCode::Char('0')) {
                let _ = writeln_str("← back");
                break MenuChoice::Back;
            }

            let KeyCode::Char(c) = code else {
                continue;
            };
            let Some(digit) = c.to_digit(10) else {
                continue;
            };
            if digit == 0 {
                continue;
            }

            let mut value = digit;
            print!("{c}");
            let _ = io::stdout().flush();

            // Wait for more digits when ambiguous (e.g. [1] vs [10–13]).
            while can_extend(value, max) {
                if !event::poll(Duration::from_millis(MULTI_DIGIT_WAIT_MS)).unwrap_or(false) {
                    break; // idle timeout → accept what we have (e.g. 1 = Solana)
                }
                let Ok(Event::Key(KeyEvent { code, kind, .. })) = event::read() else {
                    break;
                };
                if !is_press(kind) {
                    continue;
                }
                match code {
                    KeyCode::Enter => break,
                    KeyCode::Char(next) => {
                        if let Some(d) = next.to_digit(10) {
                            let next_val = value * 10 + d;
                            if next_val <= max {
                                value = next_val;
                                print!("{next}");
                                let _ = io::stdout().flush();
                            }
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }

            println!();
            if (min..=max).contains(&value) {
                break MenuChoice::Selected(value);
            }
            // Out of range — clear and wait for another key.
            print!("\r{prompt}");
            let _ = io::stdout().flush();
        }
    };
    disable_raw_mode().expect("terminal restore");
    choice
}

fn is_enter(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('\n')
    )
}

/// Read y/n — `y` confirms, `n`/Esc declines. Esc may mean “back” when `esc_is_back`.
/// `enter_confirms`: Enter / Return key counts as Yes (e.g. summary “start grinding”).
pub fn read_yes_no_key(prompt: &str, esc_is_back: bool, enter_confirms: bool) -> Option<bool> {
    print!("{prompt}");
    let _ = io::stdout().flush();

    enable_raw_mode().expect("terminal raw mode");
    let result = loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        })) = event::read()
        {
            if !is_press(kind) {
                continue;
            }

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
                code if is_enter(code) => {
                    if enter_confirms {
                        let _ = writeln_str("↵");
                    } else {
                        println!();
                    }
                    break Some(enter_confirms);
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

fn writeln_str(s: &str) -> io::Result<()> {
    println!("{s}");
    io::stdout().flush()
}
