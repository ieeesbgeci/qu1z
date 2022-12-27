use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::App;

pub fn home_ui<B: Backend>(f: &mut Frame<B>, app: &App, div: Rect) {
    //Layout
    //
    //---------------------------
    //         title            |
    //                          |
    //         desc             |
    //--------------------------|
    //          |               |
    // login    |   Guidelines  |
    //          |               |
    //          |               |
    //--------------------------|
    //
    let mut title_divs = Layout::default()
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Min(4),
        ])
        .split(div);
    //Title
    let style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::ITALIC)
        .add_modifier(Modifier::BOLD);
    let span = Span::styled("IEEE CS SBC Qu1z ::<>", style);
    let title_block = Block::default()
        .borders(Borders::NONE)
        .title_alignment(Alignment::Center)
        .title(span);
    f.render_widget(title_block, title_divs[1]);
    let desc = "Qu1z is a TUI Quiz tool used by IEEE CS SBC GECI to conduct Quiz competition : ) ";
    let desc_style = Style::default().fg(Color::White);
    let title_desc = Paragraph::new(desc)
        .alignment(Alignment::Center)
        .style(desc_style);
    f.render_widget(title_desc, title_divs[2]);
    let blk = Block::default().borders(Borders::ALL);
    f.render_widget(blk, title_divs[3]);
    //login and guidelines div
    let bottom_divs = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Min(2)])
        .split(title_divs[3]);
    let btm_style = Style::default().fg(Color::Green);
    let login_span = Span::styled("Student Login", btm_style);
    let login_blk = Block::default().title(login_span).borders(Borders::ALL);
    f.render_widget(login_blk, bottom_divs[0]);
    let guide_span = Span::styled("GuideLines", btm_style);
    let guide_blk = Block::default().title(guide_span).borders(Borders::ALL);
    f.render_widget(guide_blk, bottom_divs[1]);
    let guide_vec = App::get_guidelines();
}
pub fn qu1z_ui<B: Backend>(f: &mut Frame<B>, app: &App, div: Rect) {
    let block = Block::default()
        .title("Quiz Starts here")
        .borders(Borders::ALL);
    f.render_widget(block, div);
}
pub fn explore_ui<B: Backend>(f: &mut Frame<B>, app: &App, div: Rect) {
    let block = Block::default()
        .title("Explore More ")
        .borders(Borders::ALL);
    f.render_widget(block, div);
}
