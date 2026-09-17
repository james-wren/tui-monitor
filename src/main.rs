use ratatui::{Frame, DefaultTerminal};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block};

mod data_get;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, process] = main.layout(&horizontal);

    let left_vert = Layout::vertical([Constraint::Percentage(25); 4]).spacing(1);

    let title = Line::from_iter([
        Span::from("MacMon").bold(),
        Span::from(" (press 'q' to quit)"),
    ]);

    let [cpu, ram, gpu, disk] = left.layout(&left_vert);
    
    frame.render_widget(title.centered(), top);

    render_box(frame, cpu, "Ram");
    render_box(frame, ram, "Cpu");
    render_box(frame, gpu, "Gpu");
    render_box(frame, disk, "Disk");
    render_box(frame, process, "Processes");
}

pub fn render_box(frame: &mut Frame, area: Rect, title: &str) {
    let block = Block::bordered().title(title);
    frame.render_widget(block, area);
}