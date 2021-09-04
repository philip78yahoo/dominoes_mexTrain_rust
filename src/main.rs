
mod dominoe;
mod dominoe_deck;
mod dominoe_hand;
mod dominoe_game;
mod dominoe_train;
mod dominoe_player;

fn main() {
    println!("Hello, mexican train!");
    // let dominoe1 = dominoe::Dominoe::new(3,5);

    let game = dominoe_game::DominoeGame::new();

    println!("debug of game: {:?}", game);
    game.play();

}
