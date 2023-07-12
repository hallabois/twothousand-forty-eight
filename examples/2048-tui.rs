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

use twothousand_forty_eight::{
    board::Board, rules::ClassicV2, v2::game::replay_moves, v2::recording::SeededRecording,
};
struct GameState {
    board: Board,
    move_count: usize,
    score: usize,
    high_score: usize,
    game_over: bool,
    message: String,
    history: SeededRecording,
}
impl GameState {
    pub fn new(message: Option<&str>) -> Self {
        let random_seed = rand::random();
        let history = SeededRecording::empty(random_seed, 4, 4);
        let rules = Box::new(ClassicV2);
        let move_count = history.moves.len();
        let board = *replay_moves(&history, rules)
            .unwrap()
            .history
            .last()
            .unwrap();

        Self {
            board,
            score: 0,
            move_count,
            high_score: 0,
            game_over: false,
            message: message.unwrap_or_default().to_string(),
            history,
        }
    }
    pub fn save(&self) {
        let history_string: String = (&self.history).into();
        std::fs::write("savegame.txt", history_string).unwrap();
    }
}
impl Default for GameState {
    fn default() -> Self {
        // check if savegame.txt exists
        // if it does, load it
        // if it doesn't, create a new game
        if std::path::Path::new("savegame.txt").exists() {
            let history_string = std::fs::read_to_string("savegame.txt").unwrap();
            let history: SeededRecording = match history_string.parse() {
                Ok(history) => history,
                Err(e) => {
                    return Self::new(Some(&format!("Error parsing history: {:?}", e)));
                }
            };
            let replay = match history.reconstruct() {
                Ok(replay) => replay,
                Err(e) => {
                    return Self::new(Some(&format!("Error reconstructing history: {:?}", e)));
                }
            };
            let board = *replay.history.last().unwrap();
            return Self {
                board,
                score: replay.validation_data.score_end,
                move_count: history.moves.len(),
                high_score: 0,
                game_over: false,
                message: String::from("Loaded game from savegame.txt"),
                history,
            };
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
    run(&mut terminal, GameState::default()).context("app loop failed")?;
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
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut gamestate: GameState) -> Result<()> {
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
                    let hiscore = gamestate.high_score;
                    gamestate = GameState::new(None);
                    gamestate.high_score = hiscore;
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

fn move_in_direction(
    gamestate: &mut GameState,
    direction: twothousand_forty_eight::direction::Direction,
) {
    let validation_data = gamestate.history.validate().unwrap();
    let rules = gamestate.history.get_ruleset();
    let can_break = twothousand_forty_eight::rules::can_break(
        rules.as_ref(),
        &gamestate.board,
        validation_data.score,
        validation_data.breaks,
    );

    let result = twothousand_forty_eight::board::check_move(gamestate.board, direction);
    match result {
        Ok(result) => {
            gamestate.message = String::new();
            gamestate.board = result.board;
            gamestate.score += result.score_gain;
            gamestate.high_score = gamestate.high_score.max(gamestate.score);
            gamestate.game_over = false;

            twothousand_forty_eight::add_random_to_board(&mut gamestate.board);
            gamestate.move_count += 1;

            gamestate.history.moves.push(direction);
            let history_string: String = (&gamestate.history).into();
            let history: SeededRecording = history_string.parse().unwrap();
            assert_eq!(history, gamestate.history);
            gamestate.board = *history.reconstruct().unwrap().history.last().unwrap();
        }
        Err(e) => {
            gamestate.message = format!("{:?}", e);
        }
    }
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>, gamestate: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(2),
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
        .split(chunks[6]);

    let title = Paragraph::new(format!(
        "2048 {}x{}",
        gamestate.board.width, gamestate.board.height
    ))
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
    frame.render_widget(title, chunks[0]);
    let score = Paragraph::new(format!("Score: {}", gamestate.score));
    frame.render_widget(score, chunks[1]);
    let high_score = Paragraph::new(format!("High Score: {}", gamestate.high_score));
    frame.render_widget(high_score, chunks[2]);
    let seed = Paragraph::new(format!(
        "Seed/State: {}, Move {}",
        gamestate.board.rng_state, gamestate.move_count
    ));
    frame.render_widget(seed, chunks[3]);
    let validation_data = gamestate.history.validate().unwrap();
    let info = Paragraph::new(format!("{:?}", validation_data));
    frame.render_widget(info, chunks[4]);
    let message = Paragraph::new(format!("{}", gamestate.message));
    frame.render_widget(message, chunks[5]);
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
    let idboard = Table::new(gamestate.board.tiles.iter().map(|row| {
        Row::new(
            row.iter()
                .map(|tile| match tile {
                    Some(tile) => Cell::from(format!("{}", tile.id)).style(get_tile_style(tile)),
                    None => Cell::from("?"),
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
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ]);
    frame.render_widget(board, boardchunks[0]);
    frame.render_widget(idboard, boardchunks[1]);
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