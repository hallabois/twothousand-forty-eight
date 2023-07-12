pub const GAME3X3: &str = include_str!("../../../games/v1/GAME3X3A.txt");
pub const GAME3X3B: &str = include_str!("../../../games/v1/GAME3X3B.txt");
pub const GAME4X4: &str = include_str!("../../../games/v1/GAME4X4.txt");
pub const GAME4X4B: &str = include_str!("../../../games/v1/GAME4X4B.txt");
pub const GAME4X4C: &str = include_str!("../../../games/v1/GAME4X4C.txt");
pub const GAMEOBSCENE: &str = include_str!("../../../games/v1/GAME4X4BIG.txt");

pub const GAMELIST: &str = include_str!("../../../games/v1/composite.txt");
pub fn get_random_game() -> &'static str {
    let mut rng = rand::thread_rng();
    let games: Vec<&str> = GAMELIST.split('\n').collect();
    let index = rand::Rng::gen_range(&mut rng, 0..games.len());
    games[index]
}
