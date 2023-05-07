pub mod proc_gen;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{cmp::Ordering, error::Error, io, time::{Duration, Instant}};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, BarChart, Sparkline},
    Frame, Terminal,
};

enum State {
    Continuous,
    Manual
}

/// App holds the state of the application
struct App<'a> {
    // Input state
    state: State,
    // Generation time in milliseconds
    gen_time: Duration,
    // History of recorded messages
    map_rows: Vec<String>,
    // Map instance
    map: proc_gen::map::Map,
    // ASCII rendered on Map
    textures: proc_gen::map::TexturePack,
    // Number of rooms
    room_data: Vec<(&'a str, u64)>,
    // Time data
    time_data: Vec<u64>
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            state: State::Manual,
            gen_time: Duration::default(),
            map_rows: Vec::<String>::new(),
            map: proc_gen::map::Map::default(),
            textures: proc_gen::map::TexturePack::default(),
            room_data: Vec::<(&'a str, u64)>::new(),
            time_data: Vec::<u64>::new()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    proc_gen::heuristic(&mut app.map, &app.textures);

    // create app and run it
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        match app.state {
            State::Continuous => {
                update(&mut app);
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('g') => {
                            app.state = State::Manual;
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                        _ => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            },
            State::Manual => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                            KeyCode::Char('r') => {
                                update(&mut app);
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Char('g') => {
                                app.state = State::Continuous;
                                std::thread::sleep(std::time::Duration::from_millis(500));
                            }
                            _ => {}
                    }
                }
            }
        }
    }
}

fn update(app: &mut App) {
    // generate new map and measure time
    let start = Instant::now();
    proc_gen::heuristic(&mut app.map, &app.textures);
    let duration = start.elapsed();
    app.gen_time = duration;

    // update time data
    match app.time_data.len().cmp(&115) {
        Ordering::Greater => {
            app.time_data.remove(0);
            app.time_data.push(duration.as_micros() as u64);
        },
        _ => { app.time_data.push(duration.as_micros() as u64); }
    }

    // update num room data
    match app.room_data.len().cmp(&15) {
        Ordering::Greater => {
            app.room_data.remove(0);
            app.room_data.push(("", app.map.num_rooms() as u64));
        },
        _ => { app.room_data.push(("", app.map.num_rooms() as u64)); }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // define ui layout
    let data_map_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50)
            ].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70)
        ].as_ref())
        .split(data_map_chunks[0]);

    // create help message
    let (msg, style) = match app.state {
        State::Manual => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to generate a new map, "),
            Span::styled("g", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to continuously generate.")
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK)
        ),
        State::Continuous => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("g", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to stop continuously generating.")
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK)
        ),
    };

    // render help message
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, left_chunks[0]);

    // render time
    let time_text = format!("This generation took {:?}...", app.gen_time);
    let time_render = Paragraph::new(time_text.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Time"));
    f.render_widget(time_render, left_chunks[1]);

    // render map
    app.map.update_map_rows(&mut app.map_rows);
    let map_render: Vec<ListItem> = app
        .map_rows
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(map_render).block(Block::default().borders(Borders::ALL).title("Map"));
    f.render_widget(messages, data_map_chunks[1]);

    // render num rooms
    let barchart = BarChart::default()
        .block(Block::default().title("Room Data").borders(Borders::ALL))
        .data(&app.room_data)
        .bar_width(5)
        .bar_gap(3)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, left_chunks[3]);

    // render sparkline
    let sparkline = Sparkline::default()
        .block(Block::default().title("Time Data").borders(Borders::ALL))
        .data(&app.time_data)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, left_chunks[2]);
}