use std::{ io::{ self, Write }, error::Error, fmt::format };
use crossterm::{
    cursor::{ MoveUp, MoveDown, EnableBlinking, Hide, Show },
    execute,
    style::{ Color, Print, ResetColor, SetForegroundColor },
    event::{ KeyCode, KeyEvent, KeyEventKind, read, Event },
};

use crate::configs::Configuration;

mod configs;

#[derive(PartialEq, Clone, Copy)]
pub enum Selection {
    Yes,
    No,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    execute!(io::stdout(), EnableBlinking)?;

    // Project name setup
    let mut project_name = String::from("my-app");
    print!("What is your project named? (default: my-app)");
    print_gray_separator()?;
    io::stdout().flush()?;
    stdin.read_line(&mut project_name)?;

    // Typescript setup
    let mut typescript_selected_option = Selection::Yes;
    print_selection(
        &mut typescript_selected_option,
        "Would you like to use TypeScript with this project? » "
    )?;

    // ESLint setup
    let mut eslint_selected_option = Selection::Yes;
    print_selection(
        &mut eslint_selected_option,
        "Would you like to use ESLint with this project? » "
    )?;

    // Tailwind CSS setup
    let mut tailwindcss_selected_option = Selection::Yes;
    print_selection(
        &mut tailwindcss_selected_option,
        "Would you like to use Tailwind CSS with this project? » "
    )?;

    // Src folder structure setup
    let mut src_selected_option = Selection::No;
    print_selection(
        &mut src_selected_option,
        "Would you like to use `src/` directory with this project? » "
    )?;

    // App router setup
    let mut app_router_selected_option = Selection::Yes;
    print_selection(&mut app_router_selected_option, "Use App Router (recommended)? » ")?;

    // Import alias setup
    let mut alias_customized_selected_option = Selection::No;
    print_selection(
        &mut alias_customized_selected_option,
        "Would you like to customize the default import alias? » "
    )?;

    configs::generate_project_structure(
        &(Configuration {
            project_name: &project_name,
            is_typescript: typescript_selected_option,
            is_eslint: eslint_selected_option,
            is_tailwind: tailwindcss_selected_option,
            is_src: src_selected_option,
            is_app_router: app_router_selected_option,
            is_customize_alias: alias_customized_selected_option,
        })
    );

    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print("Project Created! \n"),
        ResetColor
    )?;

    let display_cd_instruction = format!("RUN cd {}", &project_name);
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print(display_cd_instruction),
        ResetColor
    )?;

    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print("RUN npm install \n"),
        ResetColor
    )?;

    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print("RUN npm run dev \n"),
        ResetColor
    )?;

    Ok(())
}

fn print_gray_separator() -> Result<(), Box<dyn std::error::Error>> {
    execute!(io::stdout(), SetForegroundColor(Color::DarkGrey), Print(" » "), ResetColor)?;
    Ok(())
}

fn print_selection(
    selected_option: &mut Selection,
    print_string: &str
) -> Result<(), Box<dyn Error>> {
    let mut option = if *selected_option == Selection::Yes { "Yes" } else { "No" };
    loop {
        // Reprint the updated content
        println!("{print_string} {}", if *selected_option == Selection::No {
            format!("\x1B[4m{}\x1B[0m / {}", option, "Yes")
        } else {
            format!("{} / \x1B[4m{}\x1B[0m", "No", option)
        });

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
                                *selected_option = Selection::No;
                                option = "No";
                            }
                            KeyCode::Right => {
                                *selected_option = Selection::Yes;
                                option = "Yes";
                            }
                            _ => (),
                        }
                    _ => (),
                }
            _ => (),
        }
    }
    execute!(io::stdout(), MoveDown(1), Show)?;
    Ok(())
}
