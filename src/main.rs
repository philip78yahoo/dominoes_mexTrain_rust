
mod dominoe;
mod dominoe_deck;
mod dominoe_hand;
mod dominoe_game;

fn main() {
    println!("Hello, mexican train!");
    // let dominoe1 = dominoe::Dominoe::new(3,5);

    let mut game = dominoe_game::DominoeGame::new();

    println!("debug of game: {:?}", game);

    println!("one Dominoe from game: {:?}",game.pull_from_hand(0));

    

}
