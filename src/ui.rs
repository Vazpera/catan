use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, BorderType, Paragraph},
    Frame,
};
use std::sync::{Arc, Mutex};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: Arc<App>, frame: &mut Frame) {
    let title = Title::default().content(if *app.is_host.lock().unwrap() {
        "HOST"
    } else {
        "CLIENT"
    });
    if *app.is_host.lock().unwrap() {
        let counter_span = Line::default().spans(vec![format!(
            "Value to be guessed: {}",
            *app.counter.lock().unwrap()
        )]);
        let guess_span = Line::default().spans(vec![format!(
            "Nuumber of connections = {}",
            app.connections.lock().unwrap()
        )]);

        let paragraph = Paragraph::new(vec![counter_span, guess_span]).block(
            Block::bordered()
                .title(title)
                .border_style(Style::new().blue()),
        );

        frame.render_widget(paragraph, frame.area());
    } else {
        let counter_span =
            Line::default().spans(vec![format!("Guess: {}", *app.counter.lock().unwrap())]);
        let guess_span = Line::default().spans(vec![format!(
            "{}",
            match *app.guessed_correct.lock().unwrap() {
                Some(j) => format!(
                    "Your guess was {}!",
                    if j { "right" } else { "wrong" }
                ),
                None => "Haven't guessed yet!".to_owned(),
            }
        )]);
        let paragraph = Paragraph::new(vec![counter_span, guess_span]).block(
            Block::bordered()
                .title(title)
                .border_style(Style::new().red()),
        );

        frame.render_widget(paragraph, frame.area());
    }
}
