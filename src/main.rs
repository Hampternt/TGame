pub mod lib {
    pub mod data_types;
}
pub use lib::data_types::CharacterStats;

fn main() {
    println!("Hello, world!");
    let mut _player1 = CharacterStats::default();
}
