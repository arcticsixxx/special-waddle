use std::process::exit;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem, Paragraph},
};
use tokio::task;

use crate::tui::app::TuiApp;

pub fn render(frame: &mut Frame, app: &TuiApp) {
    let layout = Layout::vertical([
        Constraint::Min(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ]);
    let [task_list_area, active_task_area, input_area] = frame.area().layout(&layout);

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| ListItem::from(format!("[ ] {}", task.title)))
        .collect();

    let list = List::new(items).block(Block::bordered());

    frame.render_widget(list, task_list_area);

    let active_task = match &app.active_task {
        Some(task) => {
            format!("Actvie task: {:?}", task.title)
        }
        None => "Acrive task: nothing".to_string(),
    };

    let active_item = Paragraph::new(active_task).block(Block::bordered());

    frame.render_widget(active_item, active_task_area);

    render_input(frame, app, input_area);
}

pub fn render_input(frame: &mut Frame, app: &TuiApp, input_area: ratatui::layout::Rect) {
    let input = Paragraph::new(app.user_input.input.as_str())
        .style(match app.user_input.input_mode {
            super::app::InputMode::Normal => ratatui::style::Style::default(),
            super::app::InputMode::Editing => {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            }
        })
        .block(Block::bordered());

    frame.render_widget(input, input_area);

    match app.user_input.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        super::app::InputMode::Normal => {}

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        #[expect(clippy::cast_possible_truncation)]
        super::app::InputMode::Editing => {
            frame.set_cursor_position(ratatui::layout::Position::new(
                // Draw the cursor at the current position in the input field.
                // This position can be controlled via the left and right arrow key
                input_area.x + app.user_input.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            ))
        }
    }
}
