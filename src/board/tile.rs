#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile{
    pub x: usize,
    pub y: usize,
    pub value: usize,
    pub merged: bool
}

impl Tile{
    pub fn oispahalla_serialize(&self) -> String{
        if self.value == 0{
            return String::from("null");
        }
        return format!("{{\"position\":{{\"x\":{},\"y\":{}}},\"value\":{}}}", self.y, self.x, self.value);
    }
}