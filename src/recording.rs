use crate::board::tile::Tile;
use crate::board::MAX_WIDTH;
use crate::board::MAX_HEIGHT;
use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Recording{
    pub width: usize,
    pub height: usize,
    pub history: Vec<( [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT], Direction, Option<Tile> )>
}

impl Recording{
    pub fn to_string(&self) -> String{
        let mut out = "".to_owned();
        let mut index: usize = 0;
        for i in self.clone().history{
            let board = crate::board::Board{tiles: i.0, width: self.width, height: self.height};
            let tiles = board.get_all_tiles();
            out = out + tiles.iter().map( |t| t.value.to_string()).collect::<Vec<String>>().join(".").as_str();
            out = out + "+";
            match i.2{
                None => out = out + "",
                Some(t) => out = out + t.x.to_string().as_str() + "," + t.y.to_string().as_str() + "." + t.value.to_string().as_str()
            }
            out = out + ";";
            out = out + i.1.get_index();
            if index < self.history.len() - 1{
                out = out + ":";
            }
            index += 1;
        }
        return out;
    }
    #[cfg(feature = "history_hash")]
    pub fn hash_v1(&self) -> String {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(self.width.to_string().as_bytes());
        hasher.update(self.height.to_string().as_bytes());
        for i in &self.history{
            hasher.update(i.1.get_index().as_bytes());
        }
        let out = format!("{:X}", hasher.finalize());
        return out;
    }
}