use crate::recording::Recording;
use crate::direction::Direction;
use crate::board::Board;
use crate::board::is_move_possible;
use crate::board::print_board;

const MAX_HISTORY_LENGTH: usize = usize::MAX;

pub fn validate_history(history: Recording) -> (bool, usize, usize) { // Valid, score, breaks
    let mut score: usize = 0;

    let history_len = history.history.len();
    let mut breaks: usize = 0;
    for ind in 0..history_len{
        let i = history.history[ind];

        let board = i.0;
        let dir = i.1;
        let addition = history.history[ind].2;

        let predicted = is_move_possible(Board { tiles: board, width: history.width, height: history.height }, dir);
        let mut predicted_board = predicted.0;
        score += predicted.2;

        if ind < (history_len - 1) && ind < MAX_HISTORY_LENGTH {
            let board_next = history.history[ind + 1].0;
            match addition{
                Some(add) => {
                    if crate::DEBUG_INFO {println!("[Add] Change {:?} => {:?}", predicted_board[add.y][add.x], add)};
                    if add.value > 4 {
                        println!("Invalid addition value at {:?}!", add);
                        return (false, 0, breaks);
                    };
                    predicted_board[add.y][add.x] = Some( add );
                },
                None => {
                    if crate::DEBUG_INFO {println!("No addition at index {}!", ind)};
                }
            }

            let board_predicted = Board{tiles: predicted_board, width: history.width, height: history.height};
            let board_actual = Board{tiles: board_next, width: history.width, height: history.height};
            if dir == Direction::END && board_predicted.get_total_value() == board_actual.get_total_value() {

            }
            else if predicted_board == board_next { // (predicted.1) && 
                
            }
            else if breaks < 3 && (board_predicted.get_total_value() > board_actual.get_total_value()) && score > 999 {
                //Kurinpalautus / Parinkulautus
                breaks += 1;
                score -= 1000;
            }
            else{
                println!("Went wrong at index {}: \n{:?}\n{:?}", ind, predicted_board, board_next);
                //println!("{:#?}", i);
                println!("Expected: (score {}) ", board_predicted.get_total_value());
                print_board(predicted_board, history.width, history.height);
                println!("Got instead: (score {}) ", board_actual.get_total_value());
                print_board(board_next, history.width, history.height);
                return (false, 0, breaks);
            }
        }
    }
    return (true, score, breaks);
}

pub fn validate_first_move(history: &Recording) -> bool {
    let history_len = history.history.len();
    if history_len > 0{
        let first_frame = history.history[0].0;
        let first_board = Board{tiles: first_frame, width: history.width, height: history.height};
        if first_board.get_total_value() < 17 {
            return true;
        }
    }
    println!("First move was not OK!");
    return false;
}

pub fn get_run_score(history: &Recording) -> usize{
    let mut score: usize = 0;
    for i in &history.history{
        let board = i.0;
        let dir = i.1;
        let predicted = is_move_possible(Board { tiles: board, width: history.width, height: history.height }, dir);
        score += predicted.2;
    }
    score
}