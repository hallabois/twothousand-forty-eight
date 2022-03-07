pub mod tile;
use tile::Tile;

pub const MAX_WIDTH: usize = 6;
pub const MAX_HEIGHT: usize = 6;

use crate::direction::Direction;

#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Board{
    pub width: usize,
    pub height: usize,
    pub tiles: [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT]
}

impl Board{
    pub fn new() -> Board{
        Board{
            width: 4,
            height: 4,
            tiles: create_tiles(4, 4)
        }
    }
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize){
        if let Some(i) = self.tiles[y][x] {
            self.tiles[y][x] = Some(Tile::new(x, y,val, i.merged));
        } else {
            if crate::DEBUG_INFO {println!("Error!")};
        }
    }
    pub fn get_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height{
            for x in 0..self.width{
                let t = self.tiles[y][x];
                match t{
                    Some(tile) => (
                        if tile.value != 0 {
                            out.push(tile)
                        }
                    ),
                    None => if crate::DEBUG_INFO {println!("Error! (pt. 2)")}
                }
            }
        }
        out
    }
    pub fn get_non_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height{
            for x in 0..self.width{
                let t = self.tiles[y][x];
                match t{
                    Some(tile) => (
                        if tile.value == 0 {
                            out.push(tile)
                        }
                    ),
                    None => if crate::DEBUG_INFO {println!("Error! (pt. 2)")}
                }
            }
        }
        out
    }
    pub fn get_all_tiles(&self) -> Vec<Tile>{
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height{
            for x in 0..self.width{
                let t = self.tiles[y][x];
                match t{
                    Some(tile) => (
                        out.push(tile)
                    ),
                    None => if crate::DEBUG_INFO {println!("Error! (pt. 2)")}
                }
            }
        }
        out
    }
    pub fn get_total_value(&self) -> usize {
        let mut sum: usize = 0;
        for row in self.tiles{
            for i in row{
                match i{
                    Some(t) => sum += t.value,
                    None => ()
                }
            }
        }
        return sum;
    }
    pub fn oispahalla_serialize(&self) -> String{
        let arr = self.tiles.iter().map(|row| row.iter().map( |t| match t{Some(t)=>t.oispahalla_serialize(),None=>String::from("null")} ).collect::<Vec<String>>() ).collect::<Vec<Vec<String>>>();
        let arr_str = format!("[{}]", arr.iter().map( |row| row.join(",") ).collect::<Vec<String>>().iter().map(|s| format!("[{}]",s)).collect::<Vec<String>>().join(","));
        let out = format!("{{\"grid\":{{\"size\":{},\"cells\":{}}},\"score\":SCOREHERE,\"palautukset\":0,\"over\":false,\"won\":false,\"keepPlaying\":false}}", usize::max(self.width, self.height) , arr_str);
        return out;
    }
}

impl Default for Board{
    fn default() -> Board{
        Board::new()
    }
}

pub fn print_board(tiles: [[Option<tile::Tile>; MAX_WIDTH]; MAX_HEIGHT], width: usize, height: usize){
    for y in 0..height{
        for x in 0..width{
            match tiles[y][x] {
                Some(i) => {
                    let string = i.value.to_string();
                    print!("{}\t", if i.value == 0 {"."} else {string.as_str()} )
                },
                None => print!("?\t")
            }
        }
        println!("");
    }
}

pub fn board_to_string(tiles: [[Option<tile::Tile>; MAX_WIDTH]; MAX_HEIGHT], width: usize, height: usize) -> String{
    let mut out = String::new();
    for y in 0..height{
        for x in 0..width{
            match tiles[y][x] {
                Some(i) => {
                    let string = i.value.to_string();
                    out += &format!("{}\t", string.as_str() );
                },
                None => {
                    out += "?\t";
                }
            }
        }
        println!("");
        out += "\n";
    }
    out
}

pub fn create_tiles(width: usize, heigth: usize) -> [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT] {
    if width > MAX_WIDTH || heigth > MAX_HEIGHT {
        panic!("Board size too big! This version of the program has been compiled to support the maximum size of {:?}", (MAX_WIDTH, MAX_HEIGHT));
    }
    let mut tiles: [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT] = [[None; MAX_WIDTH]; MAX_HEIGHT];
    for x in 0..width{
        for y in 0..heigth{
            tiles[y][x] = Some(Tile::new(x, y, 0, false));
        }
    }
    return tiles;
}

pub fn get_closest_tile(t: Tile, viable_tiles: &Vec<Tile>, dir: Direction, mask: usize) -> Tile{ //if t is returned, an error occured along the way
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();

    let mut closest = t;
    let mut closest_dist: usize = usize::MAX;
    
    let mut nearest_block = usize::MAX;

    if dir_y == 0{ // A vertical move
        for i in viable_tiles{
            let condition = if dir_x > 0 { t.x < i.x } else { t.x > i.x };
            if (t.y == i.y) && condition {
                let distance = if dir_x > 0 { i.x - t.x } else { t.x - i.x };
                if distance != 0 && distance < closest_dist {
                    let recursed = get_closest_tile(*i, viable_tiles, dir, mask);
                    if recursed != *i && recursed.value == i.value && !recursed.merged{
                        // Let this tile merge with the one in the direction of the move
                        if !recursed.merged {
                            nearest_block = distance;
                        }
                    }
                    else{
                        closest = *i;
                        closest_dist = distance;
                    }
                }
                else if distance != 0 && i.value != mask{
                    //return t;
                }
            }
        }
    }
    else { // A horizontal move
        for i in viable_tiles{
            let condition = if dir_y > 0 { t.y < i.y } else { t.y > i.y };
            if (t.x == i.x) && condition {
                let distance = if dir_y > 0 { i.y - t.y } else { t.y - i.y };
                if distance != 0 && distance < closest_dist {
                    let recursed = get_closest_tile(*i, viable_tiles, dir, mask);
                    if recursed != *i && recursed.value == i.value && !recursed.merged{
                        // Let this tile merge with the one in the direction of the move
                        if !recursed.merged {
                            nearest_block = distance;
                        }
                    }
                    else{
                        closest = *i;
                        closest_dist = distance;
                    }
                }
                else if distance != 0 && i.value != mask{
                    //return t;
                }
            }
        }
    }
    if nearest_block < closest_dist{
        return t;
    }
    return closest;
}

pub fn get_farthest_tile(t: Tile, all_tiles: &Vec<Tile>, dir: Direction, mask: usize) -> Tile{ //if t is returned, an error occured along the way
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();

    let mut farthest = t;
    let mut farthest_dist: usize = usize::MIN;

    let mut nearest_block = usize::MAX;

    if dir_y == 0{ // A vertical move
        for i in all_tiles{
            let condition = if dir_x > 0 { t.x < i.x } else { t.x > i.x };
            if (t.y == i.y) && condition {
                let distance = if dir_x > 0 { i.x - t.x } else { t.x - i.x };
                if distance != 0 && distance > farthest_dist && i.value == mask{
                    farthest = *i;
                    farthest_dist = distance;
                }
                else if distance != 0 && i.value != mask && distance < nearest_block{
                    nearest_block = distance;
                }
            }
        }
    }
    else { // A horizontal move
        for i in all_tiles{
            let condition = if dir_y > 0 { t.y < i.y } else { t.y > i.y };
            if (t.x == i.x) && condition {
                let distance = if dir_y > 0 { i.y - t.y } else { t.y - i.y };
                if distance != 0 && distance > farthest_dist && i.value == mask{
                    farthest = *i;
                    farthest_dist = distance;
                }
                else if distance != 0 && i.value != mask && distance < nearest_block{
                    nearest_block = distance;
                }
            }
        }
    }
    if nearest_block < farthest_dist{
        return t;
    }
    return farthest;
}

pub fn is_move_possible(board: Board, dir: Direction) -> ( [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT], bool, usize ) {

    if dir == Direction::END {
        return (board.tiles, true, 0);
    }

    let _tiles = board.get_occupied_tiles();

    let mut was_changed = false;

    // clone the current board
    let mut universe = create_tiles(board.width, board.height);
    for y in 0..board.height{
        for x in 0..board.width{
            match board.tiles[y][x] {
                None => if crate::DEBUG_INFO {println!("Error (pt. 6)")},
                Some(t2) => {
                    let mut t = Tile::new(t2.x, t2.y, t2.value, false);
                    #[cfg(feature = "history_hash")]
                    {
                        t.id = t2.id;
                    }
                    universe[t2.y][t2.x] = Some( t );
                }
            }
        }
    }

    let mut score = 0;

    // Merge
    let mut merged_tiles: Vec<(usize, usize)> = vec![]; // we don't want to merge a tile more than once per turn
    for _r in 0..32{
        let b = Board{tiles: universe, height: board.height, width: board.width};
        let occupied_tiles= b.get_occupied_tiles();
        //println!("Occupied tiles: {}", occupied_tiles.len());
        for t in &occupied_tiles{
            if merged_tiles.contains( &(t.x, t.y) ) || t.merged {
                // Do nothing
            }
            else{
                let closest = get_closest_tile(*t, &occupied_tiles, dir, t.value);
                if t != &closest && t.value == closest.value && !merged_tiles.contains( &(closest.x, closest.y) ) && !closest.merged {
                    
                    universe[t.y][t.x] = Some( Tile::new(t.x, t.y, 0, false) );
                    let merged = Tile::new(closest.x, closest.y, closest.value*2, true);
                    score += merged.value;
                    universe[closest.y][closest.x] = Some( merged );
                    merged_tiles.push( (merged.x, merged.y) );
                    was_changed = true;
                    if crate::DEBUG_INFO {println!("Merge {:?} + {:?} -> {:?}", t, closest, merged)};
                    break; // HOTFIX, we only want the first one before updating occupied_tiles again
                }
            }
        }
    }

    // Slide
    let mut moved_tiles: Vec<Tile> = vec![];
    for _r in 0..32{
        let b = Board{tiles: universe, width: board.width, height: board.height};
        let tiles_post = b.get_occupied_tiles();
        let _free_tiles = b.get_non_occupied_tiles();
        let all_tiles = b.get_all_tiles();
        //println!("Free tiles: {}", free_tiles.len());

        for t in &tiles_post{
            if moved_tiles.contains(t) && false{
                // Do nothing
            }
            else{
                let dir_to_use = dir;
                let farthest_free = get_farthest_tile(*t, &all_tiles, dir_to_use , 0);

                if farthest_free != *t {
                    let mut new_tile = Tile::new(farthest_free.x, farthest_free.y, t.value, false);
                    #[cfg(feature = "history_hash")]
                    {
                        new_tile.id = t.id;
                    }

                    universe[t.y][t.x] = Some( Tile::new(t.x, t.y, 0, false) );
                    universe[farthest_free.y][farthest_free.x] = Some( new_tile );
                    
                    if crate::DEBUG_INFO {println!("Move {:?} -> {:?}", t, farthest_free)};
                    moved_tiles.push(new_tile);
                    was_changed = true;
                    break; // HOTFIX, we only want the first one before updating tiles_post and free_tiles again
                }
            }
        }
    }

    for y in 0..board.height{
        for x in 0..board.width{
            match universe[y][x] {
                None => if crate::DEBUG_INFO {println!("Error (pt. 9)")},
                Some(t2) => {
                    let nt: Tile;
                    #[cfg(feature = "history_hash")]
                    {
                        nt = Tile{x: t2.x, y: t2.y, value: t2.value, merged: false, id: t2.id};
                    }
                    #[cfg(not(feature = "history_hash"))]
                    {
                        nt = Tile{x: t2.x, y: t2.y, value: t2.value, merged: false};
                    }
                    universe[y][x] = Some( nt );
                }
            }
        }
    }

    return (universe, was_changed, score);
}
