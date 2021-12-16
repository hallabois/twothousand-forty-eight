use crate::recording::Recording;
use crate::board;
use crate::direction::Direction;
use crate::board::create_tiles;

pub fn parse_data(data: String) -> Recording {
    let mut history: Vec < ( [[Option<board::tile::Tile>; board::MAX_WIDTH]; board::MAX_HEIGHT], Direction, Option<board::tile::Tile> ) > = vec![];
    let mut width = 4;
    let mut height = 4;
    let mut historypart = data.clone();
    if data.split("S").collect::<Vec<&str>>().len() > 1 {
        let parts = data.split("S").collect::<Vec<&str>>();
        historypart = parts[1].to_string();
        let dimensions = parts[0].split("x").collect::<Vec<&str>>();
        width = dimensions[0].parse::<usize>().unwrap();
        height = dimensions[1].parse::<usize>().unwrap();
    }
    for step in historypart.split(":"){
        let parts = step.split(";").collect::<Vec<&str>>();
        let bdata = parts[0].split("+").collect::<Vec<&str>>();
        let mut added = "";
        if bdata.len() > 1 {
            added = bdata[1];
        }
        let b = bdata[0];
        let mut board = create_tiles(width,height);
        let dir = parts[1];
        let direction = match dir{
            "0" => {
                Direction::UP
            },
            "1" => {
                Direction::RIGHT
            },
            "2" => {
                Direction::DOWN
            },
            "3" => {
                Direction::LEFT
            },
            _ => {
                Direction::END
            }
        };
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
    return Recording{ history, width, height };
}