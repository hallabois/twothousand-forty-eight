use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use std::io::{self, Stdout};

use twothousand_forty_eight::{unified::game::GameState, v2::recording::SeededRecording};
struct State {
    gamestate: GameState,
    history: SeededRecording,
    message: String,
    hiscore: usize,
}
impl State {
    pub fn new(message: Option<&str>) -> Self {
        let random_seed = rand::random();
        let history = SeededRecording::empty(random_seed, 4, 4);
        let gamestate = GameState::from_reconstructable_ruleset(&history).unwrap();
        Self {
            gamestate,
            history,
            message: message.unwrap_or_default().to_string(),
            hiscore: 0,
        }
    }
    pub fn save(&self) {
        let history_string: String = (&self.history).into();
        let stats = format!(
            "------- STATS -------\nScore: {}\nRNG state: {}\nBreaks: {}\nMoves: {}\nAllowed moves: {:?}\nOver: {}\nWon: {}\n------- BOARD -------\n{}\n---------------------",
            self.gamestate.score_max,
            self.gamestate.board.rng_state,
            self.gamestate.breaks,
            self.history.moves.len(),
            self.gamestate.allowed_moves,
            self.gamestate.over,
            self.gamestate.won,
            self.gamestate.board,
        );
        std::fs::write("savegame.txt", format!("{history_string}\n{stats}")).unwrap();
    }
    pub fn load(path: &str) -> Self {
        let file_str = std::fs::read_to_string(path).unwrap();
        // We only take the first line to allow comments & other cool stuff
        let history_string = file_str.lines().next().unwrap();
        let history: SeededRecording = match history_string.parse() {
            Ok(history) => history,
            Err(e) => {
                return Self::new(Some(&format!("Error parsing history: {:?}", e)));
            }
        };
        let gamestate = match GameState::from_reconstructable_ruleset(&history) {
            Ok(gamestate) => gamestate,
            Err(e) => {
                return Self::new(Some(&format!("Error reconstructing game: {:?}", e)));
            }
        };
        let hiscore = gamestate.score_max;
        Self {
            history,
            message: format!("Loaded game from {path}"),
            gamestate,
            hiscore,
        }
    }
}
impl Default for State {
    fn default() -> Self {
        const SAVE_PATH: &str = "savegame.txt";
        // check if savegame.txt exists
        // if it does, load it
        // if it doesn't, create a new game
        if std::path::Path::new(SAVE_PATH).exists() {
            return Self::load(SAVE_PATH);
        }
        Self::new(None)
    }
}

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure that the terminal is
/// restored to a sane state before exiting. This example does not do that. It also does not handle
/// events or update the application state. It just draws a greeting and exits when the user
/// presses 'q'.
fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal, State::default()).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut gamestate: State) -> Result<()> {
    loop {
        terminal.draw(|f| crate::render_app(f, &gamestate))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    // save game to savegame.txt
                    gamestate.save();
                    return Ok(());
                }
                KeyCode::Char('w') => {
                    move_in_direction(
                        &mut gamestate,
                        twothousand_forty_eight::direction::Direction::UP,
                    );
                }
                KeyCode::Char('a') => {
                    move_in_direction(
                        &mut gamestate,
                        twothousand_forty_eight::direction::Direction::LEFT,
                    );
                }
                KeyCode::Char('d') => {
                    move_in_direction(
                        &mut gamestate,
                        twothousand_forty_eight::direction::Direction::RIGHT,
                    );
                }
                KeyCode::Char('s') => {
                    move_in_direction(
                        &mut gamestate,
                        twothousand_forty_eight::direction::Direction::DOWN,
                    );
                }
                KeyCode::Char('r') => {
                    let hiscore = gamestate.hiscore;
                    gamestate = State::new(None);
                    gamestate.hiscore = hiscore;
                }
                KeyCode::Char('b') => {
                    move_in_direction(
                        &mut gamestate,
                        twothousand_forty_eight::direction::Direction::BREAK,
                    );
                }
                KeyCode::Char('o') => {
                    // save game to savegame.txt
                    gamestate.save();
                    gamestate.message = "Saved game to savegame.txt".to_string();
                }

                _ => {}
            }
        }
    }
}

fn move_in_direction(state: &mut State, direction: twothousand_forty_eight::direction::Direction) {
    if !state.gamestate.allowed_moves.contains(&direction) {
        state.message = format!("Move to direction {:?} not allowed.", direction);
        return;
    }
    let mut new_history = state.history.clone();
    new_history.moves.push(direction);
    let history_string: String = (&new_history).into();
    match history_string.parse::<SeededRecording>() {
        Ok(history) => match GameState::from_reconstructable_ruleset(&history) {
            Ok(gamestate) => {
                state.gamestate = gamestate;
                state.message = String::new();
                state.history = history;
                if state.gamestate.score_max > state.hiscore {
                    state.hiscore = state.gamestate.score_max;
                }
            }
            Err(e) => {
                state.message = format!("{:?}", e);
            }
        },
        Err(e) => {
            state.message = format!("{:?}", e);
        }
    }
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>, state: &State) {
    let gamestate = &state.gamestate;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(6),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let boardchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(23), Constraint::Min(1)].as_ref())
        .split(chunks[5]);

    let title = Paragraph::new(format!(
        "2048 {}x{}",
        gamestate.board.width, gamestate.board.height
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
    frame.render_widget(title, chunks[0]);
    let score = Paragraph::new(format!(
        "Score: {}{}",
        state.gamestate.score_max,
        if state
            .gamestate
            .allowed_moves
            .contains(&twothousand_forty_eight::direction::Direction::BREAK)
        {
            " (can break)"
        } else {
            ""
        }
    ));
    frame.render_widget(score, chunks[1]);
    let hiscore = Paragraph::new(format!("High Score: {}", state.hiscore));
    frame.render_widget(hiscore, chunks[2]);
    let seed = Paragraph::new(format!("Seed/State: {}", gamestate.board.rng_state,));
    frame.render_widget(seed, chunks[3]);
    let message = Paragraph::new(format!("{}", state.message));
    frame.render_widget(message, chunks[4]);
    let board = Table::new(gamestate.board.tiles.iter().map(|row| {
        Row::new(
            row.iter()
                .map(|tile| match tile {
                    Some(tile) => {
                        if tile.value > 0 {
                            // Align center
                            return Cell::from(format!("{}", tile.value))
                                .style(get_tile_style(tile));
                        }
                        Cell::from(".")
                    }
                    None => Cell::from(""),
                })
                .collect::<Vec<Cell>>(),
        )
    }))
    .column_spacing(1)
    .block(
        Block::default()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::all())
            .padding(ratatui::widgets::Padding::horizontal(1)),
    )
    .widths(&[
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(4),
    ]);

    frame.render_widget(board, boardchunks[0]);
}

fn get_tile_style(tile: &twothousand_forty_eight::board::tile::Tile) -> ratatui::style::Style {
    // only terminal colors are supported, ensure contrast between fg and bg
    // 2, 4 should be white on black
    // 8, 16 should be yellow on black
    // 32, 64 should be light red on black
    // 128, 256 should be red on black
    // 512, 1024 should be light blue on black
    // 2048, 4096 should be blue on black
    // 8192, 16384 should be green on black
    // 32768, 65536 should be white on black
    match tile.value {
        2 | 4 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .bg(ratatui::style::Color::Black),
        8 | 16 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::Yellow)
            .bg(ratatui::style::Color::Black),
        32 | 64 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::LightRed)
            .bg(ratatui::style::Color::Black),
        128 | 256 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::Red)
            .bg(ratatui::style::Color::Black),
        512 | 1024 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::LightBlue)
            .bg(ratatui::style::Color::Black),
        2048 | 4096 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::Blue)
            .bg(ratatui::style::Color::Black),
        8192 | 16384 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::Green)
            .bg(ratatui::style::Color::Black),
        32768 | 65536 => ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .bg(ratatui::style::Color::Black),
        _ => ratatui::style::Style::default()
            .fg(ratatui::style::Color::White)
            .bg(ratatui::style::Color::Black),
    }
}
