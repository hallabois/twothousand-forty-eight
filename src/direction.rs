#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction{
    UP,
    RIGHT,
    DOWN,
    LEFT,
    END
}
impl Direction {
    pub fn get_x(&self) -> i64{
        match self{
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 0,
            Self::LEFT => -1,
            Self::END => 0
        }
    }
    pub fn get_y(&self) -> i64{
        match self{
            Self::UP => -1,
            Self::RIGHT => 0,
            Self::DOWN => 1,
            Self::LEFT => 0,
            Self::END => 0
        }
    }
    pub fn get_index(&self) -> &str{
        match self{
            Self::UP => "0",
            Self::RIGHT => "1",
            Self::DOWN => "2",
            Self::LEFT => "3",
            Self::END => "e"
        }
    }
}