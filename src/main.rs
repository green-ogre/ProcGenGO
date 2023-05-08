/////////// ------------------------------------------------------///////////
///                                                                       ///
///                                Main                                   ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

pub mod heuristic;
pub mod cellular_automata;
pub mod drunkard;
pub mod df_aggregation;

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
    widgets::{BorderType, Block, Borders, Tabs, Paragraph, BarChart, Sparkline, ListItem, List},
    Frame, Terminal,
};


enum Algorithm {
    Heuristic,
    Cellular,
    Drunkard,
    Aggregation,
}
struct Heuristic {
    // Map instance
    map: heuristic::map::Map,
    // ASCII rendered on Map
    textures: heuristic::map::TexturePack,
}

impl Heuristic {
    fn new() -> Self {
        Heuristic { 
            map: heuristic::map::Map::new(), 
            textures: heuristic::map::TexturePack::new()
        }
    }
}

struct Cellular {
    map: cellular_automata::map::Map
}

impl Cellular {
    fn new() -> Self {
        Cellular { 
            map: cellular_automata::map::Map::new()
        }
    }
}

struct Drunkard {
    map: drunkard::map::Map
}

impl Drunkard {
    fn new() -> Self {
        Drunkard { 
            map: drunkard::map::Map::new()
        }
    }
}

struct Aggregation {
    map: df_aggregation::map::Map
}

impl Aggregation {
    fn new() -> Self {
        Aggregation { 
            map: df_aggregation::map::Map::new()
        }
    }
}

struct AlgorithmData {
    heuristic: Heuristic,
    cellular: Cellular,
    drunkard: Drunkard,
    aggregation: Aggregation,
}

impl AlgorithmData {
    fn new() -> Self {
        AlgorithmData { 
            heuristic: Heuristic::new(), 
            cellular: Cellular::new(),
            drunkard: Drunkard::new(),
            aggregation: Aggregation::new(),
        }
    }
}


struct App<'a> {
    // Generation time in microseconds
    gen_time: Duration,
    // Time data for barchart
    time_barchart: Vec<(&'a str, u64)>,
    // Time data for sparkline
    time_sparkline: Vec<u64>,
    // Generation algorithm
    algorithm: Algorithm,
    algorithm_data: AlgorithmData,
    tab_titles: Vec<&'a str>,
    tab_index: usize,
    data_list: Vec<(&'a str, String)>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            gen_time: Duration::default(),
            time_barchart: Vec::<(&'a str, u64)>::new(),
            time_sparkline: Vec::<u64>::new(),
            algorithm: Algorithm::Heuristic,
            algorithm_data: AlgorithmData::new(),
            tab_titles: vec!["Heuristic", "Cellular", "Drunkard", "Diffusion-Limited Aggregation"],
            tab_index: 0,
            data_list: Vec::<(&str, String)>::new(),
        }
    }
}

impl<'a> App<'a> {
    fn update_current_algo(&mut self) {
        match self.tab_index {
            0 => self.algorithm = Algorithm::Heuristic,
            1 => self.algorithm = Algorithm::Cellular,
            2 => self.algorithm = Algorithm::Drunkard,
            3 => self.algorithm = Algorithm::Aggregation,
            _ => self.algorithm = Algorithm::Heuristic,
        }
    }

    fn update_data_list(&mut self) {
        match self.tab_index {
            0 => heuristic::update_data_list(&self.algorithm_data.heuristic.map, &mut self.data_list),
            1 => cellular_automata::update_data_list(&self.algorithm_data.cellular.map, &mut self.data_list),
            2 => drunkard::update_data_list(&self.algorithm_data.drunkard.map, &mut self.data_list),
            3 => df_aggregation::update_data_list(&self.algorithm_data.aggregation.map, &mut self.data_list),
            _ => {}
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

    // create app and run it
    let app = App::default();
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

        match event::read()? {
            Event::Key(key) => 
            match key.code {
                KeyCode::Char('r') => {
                    update(&mut app);
                },

                KeyCode::Char('s') => {
                    match app.algorithm {
                        Algorithm::Cellular => cellular_automata::map::scramble(&mut app.algorithm_data.cellular.map),
                        Algorithm::Drunkard => drunkard::map::fill(&mut app.algorithm_data.drunkard.map),
                        _ => {}
                    }
                },

                KeyCode::Char('q') => {
                    return Ok(());
                },

                KeyCode::Char('1') => {
                    app.tab_index = 0;
                    app.update_current_algo();
                },

                KeyCode::Char('2') => {
                    app.tab_index = 1;
                    app.update_current_algo();
                },

                KeyCode::Char('3') => {
                    app.tab_index = 2;
                    app.update_current_algo();
                },

                KeyCode::Char('4') => {
                    app.tab_index = 3;
                    app.update_current_algo();
                },

                _ => {}
            },
            _ => {}
        }
    }
}

fn update(app: &mut App) {
    match app.algorithm {

        Algorithm::Heuristic => {
            // generate new map and measure time
            let start = Instant::now();
            heuristic::run(&mut app.algorithm_data.heuristic.map, &app.algorithm_data.heuristic.textures);
            let duration = start.elapsed();
            app.gen_time = duration;

            // update time data
            match app.time_sparkline.len().cmp(&115) {
                Ordering::Greater => {
                    app.time_sparkline.remove(0);
                    app.time_sparkline.push(duration.as_micros() as u64);
                },
                _ => { app.time_sparkline.push(duration.as_micros() as u64); }
            }

            // update num room data
            match app.time_barchart.len().cmp(&4) {
                Ordering::Greater => {
                    app.time_barchart.remove(0);
                    app.time_barchart.push(("HU", duration.as_micros() as u64));
                },
                _ => { app.time_barchart.push(("HU", duration.as_micros() as u64)); }
            }
        },

        Algorithm::Cellular => {
            // generate new map and measure time
            let start = Instant::now();

            cellular_automata::iterate(&mut app.algorithm_data.cellular.map);

            let duration = start.elapsed();
            app.gen_time = duration;

            // update time data
            match app.time_sparkline.len().cmp(&115) {
                Ordering::Greater => {
                    app.time_sparkline.remove(0);
                    app.time_sparkline.push(duration.as_micros() as u64);
                },
                _ => { app.time_sparkline.push(duration.as_micros() as u64); }
            }

            // update num room data
            match app.time_barchart.len().cmp(&4) {
                Ordering::Greater => {
                    app.time_barchart.remove(0);
                    app.time_barchart.push(("CA", duration.as_micros() as u64));
                },
                _ => { app.time_barchart.push(("CA", duration.as_micros() as u64)); }
            }
        }

        Algorithm::Drunkard => {
            // generate new map and measure time
            let start = Instant::now();

            drunkard::iterate(&mut app.algorithm_data.drunkard.map);

            let duration = start.elapsed();
            app.gen_time = duration;

            // update time data
            match app.time_sparkline.len().cmp(&115) {
                Ordering::Greater => {
                    app.time_sparkline.remove(0);
                    app.time_sparkline.push(duration.as_micros() as u64);
                },
                _ => { app.time_sparkline.push(duration.as_micros() as u64); }
            }

            // update num room data
            match app.time_barchart.len().cmp(&4) {
                Ordering::Greater => {
                    app.time_barchart.remove(0);
                    app.time_barchart.push(("DK", duration.as_micros() as u64));
                },
                _ => { app.time_barchart.push(("DK", duration.as_micros() as u64)); }
            }
        }

        Algorithm::Aggregation => {
            // generate new map and measure time
            let start = Instant::now();

            df_aggregation::run(&mut app.algorithm_data.aggregation.map);

            let duration = start.elapsed();
            app.gen_time = duration;

            // update time data
            match app.time_sparkline.len().cmp(&115) {
                Ordering::Greater => {
                    app.time_sparkline.remove(0);
                    app.time_sparkline.push(duration.as_micros() as u64);
                },
                _ => { app.time_sparkline.push(duration.as_micros() as u64); }
            }

            // update num room data
            match app.time_barchart.len().cmp(&4) {
                Ordering::Greater => {
                    app.time_barchart.remove(0);
                    app.time_barchart.push(("DFA", duration.as_micros() as u64));
                },
                _ => { app.time_barchart.push(("DFA", duration.as_micros() as u64)); }
            }
        }
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

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), 
            Constraint::Max(45)
            ].as_ref())
        .split(data_map_chunks[1]);

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

    let data_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([
            Constraint::Percentage(50), 
            Constraint::Percentage(50)
            ].as_ref())
        .split(left_chunks[3]);

    // create help message
    let (msg, style) = match app.algorithm {
        Algorithm::Heuristic => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to generate a new map."),
            ],
            Style::default()
        ),
        Algorithm::Cellular => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to scramble, "),
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to iterate."),
            ],
            Style::default()
        ),
        Algorithm::Drunkard => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to reset, "),
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to iterate."),
            ],
            Style::default()
        ),
        Algorithm::Aggregation => (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to generate a new map."),
            ],
            Style::default()
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
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Time Report").border_type(BorderType::Rounded));
    f.render_widget(time_render, left_chunks[1]);

    // render time barchart
    let barchart = BarChart::default()
        .block(Block::default().title("Time of Generation in Microseconds").borders(Borders::ALL).border_type(BorderType::Rounded))
        .data(&app.time_barchart)
        .bar_width(5)
        .bar_gap(3)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, data_chunks[0]);

    // render data list
    app.update_data_list();
    let map_render: Vec<ListItem> = app
        .data_list
        .iter()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(map_render).block(Block::default().borders(Borders::ALL).title("Map Data").border_type(BorderType::Rounded));
    f.render_widget(messages, data_chunks[1]);

    // render time sparkline
    let sparkline = Sparkline::default()
        .block(Block::default().title("Time Data").borders(Borders::ALL).border_type(BorderType::Rounded))
        .data(&app.time_sparkline)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, left_chunks[2]);

    // render right chunks
    let titles = app
        .tab_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::White)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Selected Algorithm")
            .border_type(BorderType::Rounded))
        .select(app.tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::RAPID_BLINK)
                .bg(Color::Black),
        );
    f.render_widget(tabs, right_chunks[0]);

    // render map
    let style = Style::default().add_modifier(Modifier::BOLD);
    let inner = {
        let mut text: Text;
        match app.tab_index {
            0 => {
                text = Text::from(format!("{}", app.algorithm_data.heuristic.map));
            },
            1 => {
                text = Text::from(format!("{}", app.algorithm_data.cellular.map));
            },
            2 => {
                text = Text::from(format!("{}", app.algorithm_data.drunkard.map));
            },
            3 => {
                text = Text::from(format!("{}", app.algorithm_data.aggregation.map));
            },
            _ => unreachable!(),
        };
        text.patch_style(style);
        Paragraph::new(text)
            .alignment(tui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
                    .title("Map")
                    .border_type(BorderType::Rounded)
            )
    };
    f.render_widget(inner, right_chunks[1]);
}