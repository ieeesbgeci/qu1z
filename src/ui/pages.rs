use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
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
            Constraint::Percentage(8),
            Constraint::Percentage(10),
            Constraint::Min(4),
        ])
        .split(div);
    f.render_widget(
        Block::default().borders(Borders::difference(Borders::ALL, Borders::BOTTOM)),
        title_divs[0],
    );
    f.render_widget(
        Block::default().borders(Borders::union(Borders::LEFT, Borders::RIGHT)),
        title_divs[1],
    );
    f.render_widget(
        Block::default().borders(Borders::difference(Borders::ALL, Borders::TOP)),
        title_divs[2],
    );
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

    // login -----

    let login_style = Style::default().fg(Color::LightRed);
    let login_span = Span::styled("< Student Login  >", btm_style);
    let login_blk = Block::default().title(login_span).borders(Borders::ALL);
    f.render_widget(login_blk, bottom_divs[0]);
    let login_div = Layout::default()
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Length(5),
        ])
        .horizontal_margin(5)
        .split(bottom_divs[0]);
    let uname_block = Block::default()
        .title(Span::styled("{ uname  }", login_style))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let sec_key_block = Block::default()
        .title(Span::styled("{ key  }", login_style))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let login_title = Paragraph::new("Enter username and Key {  } to login")
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .style(login_style);
    f.render_widget(
        login_title,
        login_div[0].inner(&Margin {
            horizontal: 0,
            vertical: 2,
        }),
    );
    f.render_widget(uname_block, login_div[1]);
    f.render_widget(sec_key_block, login_div[3]);
    //guidelines -----

    let guide_vec = App::get_guidelines();
    let guide_title = Span::styled("< GuideLines  >", btm_style);
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
            // .wrap(Wrap { trim: false })
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
                Constraint::Length(13),
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
    let quiz_divs = Layout::default()
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(55),
            Constraint::Percentage(25),
        ])
        .horizontal_margin(40)
        .split(div);
    //qu1z border
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title("Qu1z")
            .border_type(BorderType::Thick),
        div,
    );
    //questionbox border
    let question_box = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("[ Qn. {} ]", app.qn.expect("Expected Qn ").qno));

    f.render_widget(question_box, quiz_divs[1]);
    let qn_layout = Layout::default()
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(8),
            Constraint::Percentage(72),
        ])
        .margin(2)
        .split(quiz_divs[1]);
    //Qn :
    let qn = Paragraph::new(format!(
        "Qn.{}",
        app.qn.expect("Expected Question").qn.as_str()
    ))
    .block(Block::default().borders(Borders::NONE));
    f.render_widget(qn, qn_layout[0]);
    //options :
    // let opts = app.qn.opt;
    // f.render_stateful_widget()
}
pub fn explore_ui<B: Backend>(f: &mut Frame<B>, app: &App, div: Rect) {
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title("Explore more")
            .border_type(BorderType::Plain),
        div,
    );
}
