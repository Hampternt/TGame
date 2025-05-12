pub use TGame::lib::data_types::CharacterStats;
fn main() {
    println!("Hello, world!");
    let mut _player1 = CharacterStats::default(); 
    println!("{} is name yes", _player1.name());
}
