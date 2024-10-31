use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use std::sync::{Arc, Mutex};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: Arc<App>, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                {},\
                Counter: {}",
            match *app.is_host.lock().unwrap() {
                true => "SERVER".to_owned(),
                false => "CLIENT".to_owned(),
            },
            *app.counter.lock().unwrap(),
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.area(),
    );
}
