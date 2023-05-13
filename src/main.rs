/////////// ------------------------------------------------------///////////
///                                                                       ///
///                                Main                                   ///
///                                                                       ///
/////////// ------------------------------------------------------///////////

pub mod map_builders;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use map_builders::{bsp_dungeon::BSPDungeonBuilder, cellular_automata::CellularAutomataBuilder, df_aggregation::DiffusionLimitedAggregationBuilder, drunkard::DrunkardBuilder, MapBuilder};
use std::{cmp::Ordering, error::Error, io, time::{Duration}};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{BorderType, Block, Borders, Tabs, Paragraph, BarChart, Sparkline, ListItem, List},
    Frame, Terminal,
};


enum Algorithm {
    BSP,
    Cellular,
    Drunkard,
    Aggregation,
}


struct MapBuilders {
    bsp: BSPDungeonBuilder,
    cellular: CellularAutomataBuilder,
    drunkard: DrunkardBuilder,
    dfa: DiffusionLimitedAggregationBuilder,
}


struct App<'a> {
    gen_time: u128,
    time_barchart: Vec<(&'a str, u64)>,
    time_sparkline: Vec<u64>,
    algorithm: Algorithm,
    tab_titles: Vec<&'a str>,
    tab_index: usize,
    map_data: Vec<(&'a str, String)>,
    map_builders: MapBuilders,
}


impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            gen_time: 0,
            time_barchart: Vec::<(&'a str, u64)>::new(),
            time_sparkline: Vec::<u64>::new(),
            algorithm: Algorithm::BSP,
            tab_titles: vec!["(1) BSP", "(2) Cellular", "(3) Drunkard", "(4) DFA"],
            tab_index: 0,
            map_data: Vec::<(&str, String)>::new(),
            map_builders: MapBuilders {
                bsp: BSPDungeonBuilder::new(),
                cellular: CellularAutomataBuilder::new(),
                dfa: DiffusionLimitedAggregationBuilder::new(),
                drunkard: DrunkardBuilder::new(),
            }
        }
    }
}


impl<'a> App<'a> {
    fn update_current_algo(&mut self) {
        match self.tab_index {
            0 => self.algorithm = Algorithm::BSP,
            1 => self.algorithm = Algorithm::Cellular,
            2 => self.algorithm = Algorithm::Drunkard,
            3 => self.algorithm = Algorithm::Aggregation,
            _ => self.algorithm = Algorithm::BSP,
        }
        rebuild(self);
    }

    // prevents iteration on unbuilt maps which breaks app
    fn build_iter_maps(&mut self) {
        self.map_builders.cellular.build();
        self.map_builders.dfa.build();
        self.map_builders.drunkard.build();
    }

    fn set_gen_time(&mut self, gen_time: Duration) {
        self.gen_time = gen_time.as_micros();
    }

    fn update_time_charts(&mut self) {
        // update sparkline time data
        match self.time_sparkline.len().cmp(&150) {
            Ordering::Greater => { self.time_sparkline.pop(); },
            _ => {}
        }
        self.time_sparkline.insert(0, self.gen_time as u64);
    
        // update barchart time data
        let title: &str = match self.algorithm {
            Algorithm::Aggregation => "DFA",
            Algorithm::Cellular => "CELL",
            Algorithm::Drunkard => "DRU",
            Algorithm::BSP => "HUE",
        };
    
        match self.time_barchart.len().cmp(&10) {
            Ordering::Greater => { self.time_barchart.pop(); },
            _ => {}
        }
        self.time_barchart.insert(0, (title, self.gen_time as u64));
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app, initialize map builders, and run
    let mut app = App::default();
    app.build_iter_maps();
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
        // draws the current state of the app
        terminal.draw(|f| ui(f, &mut app))?;

        // handles user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('b') => {
                    rebuild(&mut app);
                },

                KeyCode::Char('i') => {
                    iterate(&mut app);
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
                }

                _ => {}
            }
        }
    }
}


fn rebuild(app: &mut App) {
    let duration = match app.algorithm {
        Algorithm::BSP => {
            let builder = &mut app.map_builders.bsp;
            let duration = map_builders::rebuild(builder);
            builder.update_map_data(&mut app.map_data);
            duration
        },
        Algorithm::Cellular => {
            let builder = &mut app.map_builders.cellular;
            let duration = map_builders::rebuild(builder);
            builder.update_map_data(&mut app.map_data);
            duration
        },
        Algorithm::Aggregation => {
            let builder = &mut app.map_builders.dfa;
            let duration = map_builders::rebuild(builder);
            builder.update_map_data(&mut app.map_data);
            duration
        },
        Algorithm::Drunkard => {
            let builder = &mut app.map_builders.drunkard;
            let duration = map_builders::rebuild(builder);
            builder.update_map_data(&mut app.map_data);
            duration
        },
    };
    app.set_gen_time(duration);

    // updates the state of the ui data for the next render
    app.update_time_charts();
}


fn iterate(app: &mut App) {
    match app.algorithm {
        Algorithm::Cellular => {
            let builder = &mut app.map_builders.cellular;
            let duration = map_builders::iterate(builder);

            builder.update_map_data(&mut app.map_data);
            if duration.as_micros() > 0 {
                app.set_gen_time(duration);
                app.update_time_charts();
            }
        },
        Algorithm::Aggregation => {
            let builder = &mut app.map_builders.dfa;
            let duration = map_builders::iterate(builder);

            builder.update_map_data(&mut app.map_data);
            if duration.as_micros() > 0 {
                app.set_gen_time(duration);
                app.update_time_charts();
            }
        },
        Algorithm::Drunkard => {
            let builder = &mut app.map_builders.drunkard;
            let duration = map_builders::iterate(builder);

            builder.update_map_data(&mut app.map_data);
            if duration.as_micros() > 0 {
                app.set_gen_time(duration);
                app.update_time_charts();
            }
        },
        _ => {}
    };
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
            Constraint::Max(50)
            ].as_ref())
        .split(data_map_chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70)
        ].as_ref())
        .split(data_map_chunks[0]);

    let data_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), 
            Constraint::Percentage(60)
            ].as_ref())
        .split(left_chunks[2]);

    let data_notes_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), 
                Constraint::Percentage(50)
            ].as_ref())
            .split(data_chunks[1]);

    // create help message
    let (msg, style) = (vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("b", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to regenerate the map, "),
            Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to iterate."),
            ],
            Style::default()
        );

    // render help message
    let mut help_text = Text::from(Spans::from(msg));
    help_text.patch_style(style);
    let time_render = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Help").border_type(BorderType::Rounded));
    f.render_widget(time_render, left_chunks[0]);

    // render time barchart
    let barchart = BarChart::default()
        .block(Block::default().title("Time of Generation in Microseconds").borders(Borders::ALL).border_type(BorderType::Rounded))
        .data(&app.time_barchart)
        .bar_width(5)
        .bar_gap(3)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, data_chunks[0]);

    // render map data list
    let map_render: Vec<ListItem> = app
        .map_data
        .iter()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(map_render).block(Block::default().borders(Borders::ALL).title("Map Data").border_type(BorderType::Rounded));
    f.render_widget(messages, data_notes_chunks[0]);

    // render misc notes
    let notes = Text::from(textwrap::fill(match app.algorithm {
        Algorithm::BSP => app.map_builders.bsp.notes(),
        Algorithm::Cellular => app.map_builders.cellular.notes(),
        Algorithm::Aggregation => app.map_builders.dfa.notes(),
        Algorithm::Drunkard => app.map_builders.drunkard.notes(),
    }, 45));
    let notes_render =
        Paragraph::new(notes).block(Block::default().borders(Borders::ALL).title("Misc Notes").border_type(BorderType::Rounded));
    f.render_widget(notes_render, data_notes_chunks[1]);

    // render time sparkline
    let sparkline = Sparkline::default()
        .block(Block::default().title("Time Data").borders(Borders::ALL).border_type(BorderType::Rounded))
        .data(&app.time_sparkline)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(sparkline, left_chunks[1]);

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
        let mut text = Text::from(match app.algorithm {
            Algorithm::BSP => format!("{}", app.map_builders.bsp.get_map()),
            Algorithm::Cellular => format!("{}", app.map_builders.cellular.get_map()),
            Algorithm::Aggregation => format!("{}", app.map_builders.dfa.get_map()),
            Algorithm::Drunkard => format!("{}", app.map_builders.drunkard.get_map()),
        });
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