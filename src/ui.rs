use crate::app::{App, CurrentScreen};
use crate::tree::TreeState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Text,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Clear, Paragraph, Wrap,
    },
    Frame,
};

pub fn ui2(frame: &mut Frame, app: &App) {
    let forest = &app.forest;
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]);
    let [top, middle, bottom] = vertical.areas(frame.area());
    {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let mut count = 0;
        for row in forest.iter() {
            for tree in row.iter() {
                if tree.state == TreeState::Burned {
                    count += 1;
                }
            }
        }

        let total_trees = forest.len() * forest[0].len();
        let percent = (count as f64 / total_trees as f64) * 100.0;
        let title_text = format!("Burned %: {}", percent);
        let title = Paragraph::new(Text::styled(title_text, Style::default().fg(Color::Green)))
            .block(title_block);

        frame.render_widget(title, top);
    }
    //Trees
    {
        //Size
        let left = 10.0;
        let right = forest[0].len() as f64 * 1.1;
        // let right = f64::from(middle.width);
        let bottom = 10.0;
        let top = forest.len() as f64 * 1.1;
        // let top = f64::from(middle.height);

        let canvas_block = Canvas::default()
            .block(Block::bordered().title("Forest"))
            .marker(symbols::Marker::Block)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                ctx.layer();
                let mut alive_trees: Vec<(f64, f64)> = Vec::new();
                let mut burned_trees: Vec<(f64, f64)> = Vec::new();
                for (y, row) in forest.iter().enumerate() {
                    for (x, tree) in row.iter().enumerate() {
                        match tree.state {
                            TreeState::Alive => alive_trees.push((y as f64, x as f64)),
                            TreeState::Burned => burned_trees.push((y as f64, x as f64)),
                            TreeState::None => {}
                        };
                    }
                }
                if !alive_trees.is_empty() {
                    ctx.draw(&Points {
                        coords: &alive_trees,
                        color: Color::Green,
                    });
                }

                if !burned_trees.is_empty() {
                    ctx.draw(&Points {
                        coords: &burned_trees,
                        color: Color::Red,
                    });
                }
            });

        frame.render_widget(canvas_block, middle);
    }
    // Footer
    {
        let footer_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let footer_text = Paragraph::new(Text::styled(
            "Press Q to exit",
            Style::default().fg(Color::Green),
        ))
        .block(footer_block);

        frame.render_widget(footer_text, bottom);
    }
    //Exit
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to exit? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
