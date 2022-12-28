use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::App;

//Newtype Guidelines Div for space btw guideline rules
//hehe : )
#[derive(Debug)]
struct GuideLinesDiv<'a>(&'a [Rect]);

impl Iterator for GuideLinesDiv<'_> {
    type Item = Rect;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() > 2 {
            let res = self.0[1];
            self.0 = &self.0[2..];
            Some(res)
        } else {
            None
        }
    }
}

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
    let title_divs = Layout::default()
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
    let guide_vec = App::get_guidelines();
    let guide_title = Span::styled("GuideLines", btm_style);
    let guide_blk = Block::default().title(guide_title).borders(Borders::ALL);
    f.render_widget(guide_blk, bottom_divs[1]);
    let guide_spans = guide_vec
        .iter()
        .map(|g| Spans::from(Span::styled(*g, Style::default())))
        .collect::<Vec<Spans>>();
    let guide_lines = guide_spans
        .into_iter()
        .map(|s| {
            Paragraph::new(s)
                .block(Block::default().borders(Borders::NONE))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false })
        })
        .collect::<Vec<Paragraph>>();
    let guide_area = GuideLinesDiv(
        &Layout::default()
            .margin(3)
            .constraints([
                Constraint::Percentage(7),
                Constraint::Percentage(8),
                Constraint::Percentage(7),
                Constraint::Percentage(8),
                Constraint::Percentage(7),
                Constraint::Percentage(8),
                Constraint::Percentage(3),
                Constraint::Percentage(8),
                Constraint::Percentage(7),
                Constraint::Percentage(8),
                Constraint::Percentage(7),
                Constraint::Percentage(8),
                Constraint::Percentage(7),
            ])
            .split(bottom_divs[1]),
    )
    .into_iter()
    .collect::<Vec<Rect>>();
    (0..6).for_each(|i| {
        f.render_widget(guide_lines[i].clone(), guide_area[i]);
    });
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
