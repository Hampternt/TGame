pub use text_game::lib::data_types::CharacterStats;
fn main() {
    println!("Hello, world!");
    let mut player1 = CharacterStats::default(); 
    println!("{} is name yes", player1.name());

    player1.set_name("Hammy".to_string());
    
    println!("{} is now mah nem.", player1.name());
}
