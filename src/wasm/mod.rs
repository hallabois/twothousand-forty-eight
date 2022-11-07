use wasm_bindgen::prelude::*;

use crate::*;

#[wasm_bindgen]
pub fn parse(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data));
    return serde_json::to_string(&parsed).unwrap();
}

#[wasm_bindgen]
pub fn get_frames(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data)).unwrap();
    let reconstruction = validator::reconstruct_history(parsed).unwrap();

    return serde_json::to_string(&reconstruction.history).unwrap();
}

#[wasm_bindgen]
pub fn validate(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data)).unwrap();
    // let first_move_valid = validator::validate_first_move(&parsed);
    let history_valid = validator::validate_history(parsed);
    return serde_json::to_string(&history_valid).unwrap();
}

#[wasm_bindgen]
pub fn validate_all_frames(data: &str) -> String {
    let frames_src = data.split(":").collect::<Vec<&str>>();
    let frame_count = frames_src.clone().len();
    println!("found {} frames", frame_count);
    let mut validation_results: Vec<
        Option<Result<validator::ValidationData, validator::ValidationError>>,
    > = vec![];

    for frame in 0..frame_count {
        let section = frames_src[0..frame].join(":");
        match parser::parse_data(section) {
            Ok(parsed) => {
                let history_valid = validator::validate_history(parsed);
                validation_results.push(Some(history_valid));
            }
            Err(_) => {
                validation_results.push(None);
            }
        }
    }
    return serde_json::to_string(&validation_results).unwrap();
}

#[wasm_bindgen]
pub fn apply_move(board_data: &str, dir: usize, add_random: bool) -> String {
    let b = serde_json::from_str(board_data).unwrap();
    // let first_move_valid = validator::validate_first_move(&parsed);
    let mut result = board::check_move(b, direction::Direction::from_index(dir));
    if add_random {
        let mut game = board::Board {
            width: b.width,
            height: b.height,
            tiles: result.tiles,
        };
        crate::add_random_to_board(&mut game);
        result.tiles = game.tiles;
    }
    return serde_json::to_string(&result).unwrap();
}

#[wasm_bindgen]
pub fn add_random(board_data: &str) -> String {
    let b: board::Board = serde_json::from_str(board_data).unwrap();
    let mut game = board::Board {
        width: b.width,
        height: b.height,
        tiles: b.tiles,
    };
    add_random_to_board(&mut game);
    return serde_json::to_string(&game.tiles).unwrap();
}

#[cfg(feature = "history_hash")]
#[wasm_bindgen]
pub fn hash(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data)).unwrap();
    return serde_json::to_string(&parsed.hash_v1()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::validate_all_frames as wasm_validate_all_frames;
    use crate::tests::lib_testgames;
    use crate::validator;

    // This test is quite slow (a lot of json parsing)
    #[test]
    fn validate_all_frames() {
        let validation_result = wasm_validate_all_frames(lib_testgames::GAME3X3);
        let parsed: Vec<Option<Result<validator::ValidationData, validator::ValidationError>>> =
            serde_json::from_str(&validation_result).unwrap();
        println!("parsed: {:?}", parsed);
        println!("parsed length: {}", parsed.len());

        let first = parsed.get(0).unwrap();
        println!("first: {:?}", first);
        assert!(first.is_none());
        let last = parsed.last().unwrap().clone();
        println!("last: {:?}", last);
        let unwrapped = last.unwrap();
        let unwrapped = unwrapped.unwrap();
        assert_eq!(unwrapped.score, 14212);
        assert_eq!(unwrapped.breaks, 0);
        assert_eq!(unwrapped.score_end, 14212);
        assert_eq!(unwrapped.score_margin, 64);
    }
}