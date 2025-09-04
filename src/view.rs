use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal) // or Vertical if you want top/bottom
        .constraints([
            Constraint::Percentage(33), // 1/3 of the screen
            Constraint::Percentage(67), // remaining 2/3
        ])
        .split(frame.area());

    let title = Line::from("Users").left_aligned().bold();
    let text = "test";

    frame.render_widget(
        Paragraph::new(text).block(Block::bordered().title(title)),
        chunks[0], // only render in the first third
    );
}
