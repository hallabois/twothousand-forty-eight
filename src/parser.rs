use crate::recording::Recording;
use crate::recording::History;
use crate::board;
use crate::direction::Direction;
use crate::board::create_tiles;

/// Parses a string representation of a played 2048 game into a [Recording]
/// 
/// The string should be in the following format:
/// - The string can start with ```[w]x[h]S```, specifying the size of the game board, otherwise it defaults to 4 by 4.
/// (w = width, h = height, both are a [usize])
/// - History indicies should be separated by a single ```:```
///     - ```;``` separates the history index to:
///         - The board data of the move on the left side, separated by a ```+```:
///             - Each tile value of the board, separated from eachother by dots (```.```) on the left side
///             - The tile added to the board after this move on the right side
///                 - The tile's value is on the right side of a comma (```,```) and it's position is on the left side of the comma, separated by a dot (```.```), x first
///         - [Direction] index of the move on the right side
/// 
/// e.g. ```4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1```
pub fn parse_data(data: String) -> Option<Recording> {
    let mut history: History = vec![];
    let mut width = 4;
    let mut height = 4;
    let mut historypart = data.clone();
    if data.split("S").collect::<Vec<&str>>().len() > 1 {
        let parts = data.split("S").collect::<Vec<&str>>();
        historypart = parts.get(1)?.to_string();
        let dimensions = parts[0].split("x").collect::<Vec<&str>>();
        width = dimensions.get(0)?.parse::<usize>().unwrap();
        height = dimensions.get(1)?.parse::<usize>().unwrap();
    }
    for step in historypart.split(":"){
        let parts = step.split(";").collect::<Vec<&str>>();
        let bdata = parts[0].split("+").collect::<Vec<&str>>();
        let mut added = "";
        if bdata.len() > 1 {
            added = bdata[1];
        }
        let b = *bdata.get(0)?;
        let mut board = create_tiles(width,height);
        let dir = parts.get(1)?;
        let direction = Direction::from_index_str(dir);
        let mut index: usize = 0;
        for i in b.split("."){
            let val = i.parse::<usize>().unwrap();
            let x = index % width;
            let y = index / height;
            board[ y ][ x ] = Some ( board::tile::Tile::new(x, y, val, false) );
            index += 1;
        }

        let mut added_tile = None;
        if added != ""{
            let added_vals = added.split(".").collect::<Vec<&str>>();
            let added_index = added_vals[0];
            let added_pos = added_index.split(",").collect::<Vec<&str>>();
            let added_x = added_pos[0].parse::<usize>().unwrap();
            let added_y = added_pos[1].parse::<usize>().unwrap();
            let added_value = added_vals[1].parse::<usize>().unwrap();
            added_tile = Some( board::tile::Tile::new( added_x, added_y , added_value, false ) );
        }
        
        history.push( (board, direction, added_tile) );
    }
    return Some(Recording{ history, width, height });
}