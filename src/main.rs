use std::{error::Error as Err, time::Duration};
mod app;
mod ui;
use app::models::Question;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};
use ui::app::{run_app, App};
fn main() -> Result<(), Box<dyn Err>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let t_rate = Duration::from_millis(250);
    let name = String::from("vazha");
    let qn_str = std::fs::read_to_string("test.json")?;
    let qn: Question = serde_json::from_str(&qn_str)?;
    let mut app = App::new(&name);
    app.set_qn(&qn);
    run_app(&mut term, app, t_rate)?;
    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    Ok(())
}
