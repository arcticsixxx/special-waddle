use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::tui::app::TuiApp;

pub fn render(frame: &mut Frame, app: &TuiApp) {
    let areas = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(frame.area());

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| ListItem::from(format!("[ ] {}", task.title)))
        .collect();

    let list = List::new(items).block(Block::bordered());

    frame.render_widget(list, areas[0]);

    let active_task = match &app.active_task {
        Some(task) => {
            format!("Actvie task: {:?}", task.title)
        }
        None => "Acrive task: nothing".to_string(),
    };

    let active_item = Paragraph::new(active_task).block(Block::bordered());

    frame.render_widget(active_item, areas[1]);
}
