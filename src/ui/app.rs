use std::{
    error::Error as Err,
    thread,
    time::{Duration, Instant},
};

use super::pages::{explore_ui, home_ui, qu1z_ui};
use crossbeam_channel::bounded;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

#[derive(Debug)]
pub struct App<'a> {
    user: User<'a>,
    app_state: AppState,
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
}

#[derive(Debug)]
enum AppState {
    Home,
    Qu1z,
    Explore,
}

#[derive(Debug)]
enum InputDirection {
    Left,
    Right,
}

impl From<&AppState> for usize {
    fn from(state: &AppState) -> Self {
        use AppState::*;
        match state {
            Home => 0,
            Qu1z => 1,
            Explore => 2,
        }
    }
}
impl From<&AppState> for String {
    fn from(state: &AppState) -> Self {
        use AppState::*;
        match state {
            Home => "Home".into(),
            Qu1z => "Qu1z".into(),
            Explore => "Explore".into(),
        }
    }
}
impl From<usize> for AppState {
    fn from(state: usize) -> Self {
        use AppState::*;
        match state {
            0 => Home,
            1 => Qu1z,
            _ => Explore,
        }
    }
}

impl<'a> App<'a> {
    pub fn new(name: &'a str) -> Self {
        App {
            user: User { name },
            app_state: AppState::Home,
        }
    }
    pub fn change_state(&mut self, input: InputDirection) {
        let cur_page = usize::from(&self.app_state);
        use InputDirection::*;
        let req_page: usize = match input {
            Left if cur_page > 0 => cur_page - 1,
            Left => 2,
            Right => (cur_page + 1) % 3,
        };
        self.app_state = AppState::from(req_page);
    }
    pub fn get_guidelines() -> Vec<String> {
        vec![
            "Use h,l or ArrowKeys to navigate btw tabs".into(),
            "All Students should use their real name as Username".into(),
            "Participants are requested to enter secret key recieved while registering in web platform"
                .into(),
            "Participants are requested to avoid malpractices".into(),
            "Submissions with malpractices will be dealt seriously".into(),
        ]
    }
}

pub fn run_app<B: Backend>(
    term: &mut Terminal<B>,
    mut app: App,
    t_rate: Duration,
) -> Result<(), Box<dyn Err>> {
    let (tx, rx) = bounded::<Event>(5);
    thread::spawn(move || {
        let mut last_t = Instant::now();
        loop {
            let timeout = t_rate
                .checked_sub(last_t.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    tx.send(Event::Key(key)).unwrap();
                }
            }
            if last_t.elapsed() >= t_rate {
                last_t = Instant::now();
            }
        }
    });
    loop {
        term.draw(|f| draw_ui(f, &mut app));
        if let Event::Key(key) = rx.recv()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Char('h') | KeyCode::Left => app.change_state(InputDirection::Left),
                KeyCode::Char('l') | KeyCode::Right => app.change_state(InputDirection::Right),
                _ => {}
            };
        };
    }
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let div = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());
    let title = ["Home", "Qu1z", "Explore"]
        .into_iter()
        .map(|t| Spans::from(Span::styled(t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(title)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red))
        .select(usize::from(&app.app_state));
    f.render_widget(tabs, div[0]);
    use AppState::*;
    match app.app_state {
        Home => home_ui(f, &app, div[1]),
        Qu1z => qu1z_ui(f, &app, div[1]),
        Explore => explore_ui(f, &app, div[1]),
    };
}
