use std::thread;
use std::time::Duration;

use color_eyre::eyre::Ok;
use ratatui::{Frame, DefaultTerminal};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Stylize, Color};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, RenderDirection, Sparkline};
use sysinfo::System;

mod data_get;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut sys = System::new_all();

    let max_ram = sys.total_memory();

    let mut memory: Vec<u64> = vec![];
    let mut cpu: Vec<u64> = vec![];

    loop {
        memory.insert(0, data_get::get_info("ram", &mut sys)); 
        cpu.insert(0, data_get::get_info("cpu", &mut sys));
        let _disk = data_get::get_disks();

        terminal.draw(|frame| render(frame, &cpu, &memory, max_ram))?;

        thread::sleep(Duration::from_millis(500));
    }
}

fn render(frame: &mut Frame, cpu_data: &[u64], ram_data: &[u64], max_ram: u64) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, process] = main.layout(&horizontal);

    let left_vert = Layout::vertical([Constraint::Percentage(33); 3]).spacing(1);

    let title = Line::from_iter([
        Span::from("Larpy").bold(),
        Span::from(" (press ctrl + c to quit)"),
    ]);

    let [cpu, ram, disk] = left.layout(&left_vert);
    
    frame.render_widget(title.centered(), top);

    let cpu_box = render_box(frame, cpu, "Cpu");
    let ram_box = render_box(frame, ram, "Ram");
    let disk_box = render_box(frame, disk, "Disk");
    let _processes_box = render_box(frame, process, "Processes");

    render_spark(frame, cpu_box, &cpu_data, 10000);
    render_spark(frame, ram_box, &ram_data, max_ram);
    render_spark(frame, disk_box, &[10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 100);
}

pub fn render_box(frame: &mut Frame, area: Rect, title: &str) -> Rect {
    let block = Block::bordered().title(title);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

pub fn render_spark(frame: &mut Frame, area: Rect, data: &[u64], max: u64) {
    let spark_line = Sparkline::default()
        .data(data)
        .max(max)
        .direction(RenderDirection::RightToLeft)
        .style(Color::Cyan);

    frame.render_widget(spark_line, area);
}