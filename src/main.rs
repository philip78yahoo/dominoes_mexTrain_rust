
mod dominoe;
mod dominoe_deck;

fn main() {
    println!("Hello, mexican train!");
    // let dominoe1 = dominoe::Dominoe::new(3,5);

    let mut deck = dominoe_deck::DominoeDeck::new();

    println!("deck size: {}",deck.size());

    let mydominoe = deck.pull_dominoe();
    match mydominoe {
      None => println!("1st deck.pullDominoe() failed"),
      Some(mydominoe) => {
	      println!("my 1st dominoe is:");
          mydominoe.show();
      },
    }

    let mydominoe2 = deck.pull_dominoe();
    match mydominoe2 {
      None => println!("2nd deck.pullDominoe() failed"),
      Some(mydominoe2) => {
	      println!("my 2nd dominoe is:");
          mydominoe2.show();
      },
    }

    println!("my deck ( minus the 2 I already pulled and showed):");
    deck.show();

    // cant do this...let dominoe1 = dominoelist1.pop();

    // cant do this...print!("dominoe1: ");dominoe1.show();
    // cant do this...print!("dominoe2: ");dominoe2.show();

    

}