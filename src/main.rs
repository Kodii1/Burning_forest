use std::{
    io::stdout,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyEventKind},
    ExecutableCommand,
};

use crossterm::event::{self, Event, KeyCode};

use ratatui::{backend::Backend, Terminal};

mod app;
mod tree;

use crate::tree::{Tree, TreeState};
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui2,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = ratatui::init();
    let mut app = App::new(80, 500, 500);
    App::create_forest(&mut app.forest, app.percent, app.size.0, app.size.1);
    App::fire_random_tree(&mut app.forest);
    let _ = run_app(&mut terminal, &mut app);
    // let _ = debug(&mut app);
    ratatui::restore();
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}

pub fn debug(app: &mut App) -> std::io::Result<bool> {
    let tick_rate = Duration::from_millis(100);
    let last_tick = Instant::now();
    let mut x = 0;
    loop {
        x += 1;
        App::spread_fire(&mut app.forest);
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                            continue;
                        }
                        _ => {}
                    },
                }
            }
        }
        if x > 100 {
            break;
        }
    }

    print_forest(&app.forest);
    Ok(true)
}

pub fn print_forest(forest: &Vec<Vec<Tree>>) {
    for row in forest {
        for tree in row {
            let symbol = match tree.state {
                TreeState::Alive => '🎄',
                TreeState::Burned => '🔥',
                TreeState::None => '🤡',
            };

            print!(" {} |", symbol);
        }
        println!();
        println!("+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+");
        // Druga linia pozioma po wierszu
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<bool> {
    let tick_rate = Duration::from_millis(1);
    let last_tick = Instant::now();

    let mut x = 0;
    loop {
        x += 1;
        App::spread_fire(&mut app.forest);
        terminal.draw(|f| ui2(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                            continue;
                        }
                        _ => {}
                    },
                }
            }
        }

        // if x > 100 {
        //     break;
        // }
    }
    // Ok(true)
}
