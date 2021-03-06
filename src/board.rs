//! Provides [Board] to hold game board data and [tile] to hold the values of the board tiles.

pub mod tile;
use tile::Tile;
use crate::direction::Direction;
#[cfg(feature = "serde_derive")]
use serde::{Serialize, Deserialize};

/// Max width of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_WIDTH: usize = 5;

/// Max height of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_HEIGHT: usize = 5;

/// Holds game board data
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Board{
    /// The width of the board. Value of 0 is untested
    pub width: usize,

    /// The height of the board. Value of 0 is untested
    pub height: usize,

    /// The tiles of the board, note that the size of the array is allocated based on the max size.
    pub tiles: [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT]
}

impl Board{

    /// Create a new board with the height and width of 4 and initialize all tiles with [create_tiles]
    pub fn new() -> Board{
        Board{
            width: 4,
            height: 4,
            tiles: create_tiles(4, 4)
        }
    }

    /// Set a tile on the board and silently fail if the target tile doesn't exist and [DEBUG_INFO](crate::DEBUG_INFO) is disabled.
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize) {
        if let Some(i) = self.tiles[y][x] {
            self.tiles[y][x] = Some(Tile::new(x, y,val, i.merged));
        } else {
            if crate::DEBUG_INFO {println!("Error!")};
        }
    }

    /// Get the tiles that exist and which's values are nonzero
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

    /// Get the tiles that exist and which's values are zero
    pub fn get_non_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
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

    /// Get all tiles that exist ( aren't None )
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

    /// Get the combined value of all the tiles
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

    /// Gives a string representation of the board that is compatible with our anticheat systems
    #[cfg(feature = "serde_derive")]
    pub fn oispahalla_serialize(&self, score: Option<usize>) -> String{
        let score_str = match score {
            Some(s) => s.to_string(),
            None => String::from("SCOREHERE")
        };
        let arr = self.tiles.iter().map(|row| row.iter().map( |t| match t{Some(t)=>t.to_json(),None=>String::from("null")} ).collect::<Vec<String>>() ).collect::<Vec<Vec<String>>>();
        let arr_str = format!("[{}]", arr.iter().map( |row| row.join(",") ).collect::<Vec<String>>().iter().map(|s| format!("[{}]",s)).collect::<Vec<String>>().join(","));
        let out = format!("{{\"grid\":{{\"size\":{},\"cells\":{}}},\"score\":{},\"palautukset\":0,\"over\":false,\"won\":false,\"keepPlaying\":false}}", usize::max(self.width, self.height) , arr_str, score_str);
        return out;
    }
}

/// Initialize a new board with [Board::new]
impl Default for Board{
    fn default() -> Board{
        Board::new()
    }
}

/// Print a debug visualization of the board
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

/// Return a debug visualization of the board
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

/// Initialize an array of empty tiles created with [Tile::new]
pub fn create_tiles(width: usize, height: usize) -> [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT] {
    if width > MAX_WIDTH || height > MAX_HEIGHT {
        panic!("Board size too big! This version of the program has been compiled to support the maximum size of {:?}", (MAX_WIDTH, MAX_HEIGHT));
    }
    let mut tiles: [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT] = [[None; MAX_WIDTH]; MAX_HEIGHT];
    for x in 0..width{
        for y in 0..height{
            tiles[y][x] = Some(Tile::new(x, y, 0, false));
        }
    }
    return tiles;
}

/// Return the closest tile with the value of "mask" to the tile "t" in the given direction "dir", if "t" is returned, no such tile was found.
pub fn get_closest_tile(t: Tile, viable_tiles: &Vec<Tile>, dir: Direction, mask: usize) -> Tile { //if t is returned, an error occured along the way
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
    else if dir_x == 0 { // A horizontal move
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

/// Return the farthest tile with the value of "mask" to the tile "t" in the given direction "dir", if "t" is returned, no such tile was found.
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
    else if dir_x == 0 { // A horizontal move
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

/// Check if a move is possible in the direction "dir" and return the next board, the possibility, and the score gain
pub fn is_move_possible(board: Board, dir: Direction) -> ( [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT], bool, usize ) {

    if dir == Direction::END {
        return (board.tiles, true, 0);
    }

    let mut was_changed = false;

    // clone the current board
    let mut universe = create_tiles(board.width, board.height);
    for y in 0..board.height{
        for x in 0..board.width{
            match board.tiles[y][x] {
                None => if crate::DEBUG_INFO {println!("Error (pt. 6)")},
                Some(t2) => {
                    #[cfg(not(feature = "tile_id"))]
                    let t: Tile;
                    #[cfg(feature = "tile_id")]
                    let mut t: Tile;

                    t = Tile::new(t2.x, t2.y, t2.value, false);

                    #[cfg(feature = "tile_id")]
                    {
                        t.id = t2.id;
                    }
                    #[cfg(feature = "tile_merged_from")]
                    {
                        t.merged_from = t2.merged_from;
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
        for t in &occupied_tiles{
            if merged_tiles.contains( &(t.x, t.y) ) || t.merged {
                // Do nothing
            }
            else{
                let closest = get_closest_tile(*t, &occupied_tiles, dir, t.value);
                if t != &closest && t.value == closest.value && !merged_tiles.contains( &(closest.x, closest.y) ) && !closest.merged {
                    
                    universe[t.y][t.x] = Some( Tile::new(t.x, t.y, 0, false) );

                    #[cfg(not(feature = "tile_merged_from"))]
                    let merged: Tile;
                    #[cfg(feature = "tile_merged_from")]
                    let mut merged: Tile;

                    merged = Tile::new(closest.x, closest.y, closest.value*2, true);

                    #[cfg(feature = "tile_merged_from")] {
                        merged.merged_from = Some([t.id, closest.id]);
                    }

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
                    #[cfg(not(feature = "tile_id"))]
                    let new_tile: Tile;
                    #[cfg(feature = "tile_id")]
                    let mut new_tile: Tile;

                    new_tile = Tile::new(farthest_free.x, farthest_free.y, t.value, false);

                    #[cfg(feature = "tile_id")]
                    {
                        new_tile.id = t.id;
                    }
                    #[cfg(feature = "tile_merged_from")]
                    {
                        new_tile.merged_from = t.merged_from;
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
                    #[cfg(not(feature = "tile_id"))]
                    let nt: Tile;
                    #[cfg(feature = "tile_id")]
                    let mut nt: Tile;

                    nt = Tile::new(t2.x, t2.y, t2.value, false);
                    #[cfg(feature = "tile_id")]
                    {
                        nt.id = t2.id;
                    }
                    #[cfg(feature = "tile_merged_from")]
                    {
                        nt.merged_from = t2.merged_from;
                    }
                    universe[y][x] = Some( nt );
                }
            }
        }
    }

    return (universe, was_changed, score);
}
