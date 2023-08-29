use std::io::{ self, Write };
use crossterm::{
    cursor::{ MoveUp, MoveDown, EnableBlinking, Hide, Show },
    execute,
    style::{ Color, Print, ResetColor, SetForegroundColor },
    event::{ KeyCode, KeyEvent, KeyEventKind, read, Event },
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    // Project name setup
    let mut project_name = String::new();
    print!("What is your project named?");
    print_gray_separator()?;
    io::stdout().flush()?;
    stdin.read_line(&mut project_name)?;

    // Typescript setup
    let mut selected_option = 1; // 1 for "No", 2 for "Yes"
    let mut typescript_option = "No";

    loop {
        // Reprint the updated content
        print!("Would you like to use TypeScript with this project? » {}", if selected_option == 1 {
            format!("\x1B[4m{}\x1B[0m / {}", typescript_option, "Yes")
        } else {
            format!("{} / \x1B[4m{}\x1B[0m", "No", typescript_option)
        });
        println!();

        execute!(io::stdout(), MoveUp(1), Hide)?;
        match read()? {
            Event::Key(KeyEvent { code, kind, .. }) =>
                match kind {
                    KeyEventKind::Press =>
                        match code {
                            KeyCode::Enter => {
                                break;
                            }
                            KeyCode::Left => {
                                selected_option = 1;
                                typescript_option = "No";
                            }
                            KeyCode::Right => {
                                selected_option = 2;
                                typescript_option = "Yes";
                            }
                            _ => (),
                        }
                    _ => (),
                }
            _ => (),
        }
    }
    execute!(io::stdout(), MoveDown(1), Show)?;
    println!("HALOO");

    Ok(())
}

fn print_gray_separator() -> Result<(), Box<dyn std::error::Error>> {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(" >> "),
        ResetColor,
        EnableBlinking
    )?;
    Ok(())
}
