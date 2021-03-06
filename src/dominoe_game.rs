extern crate rand;

use crate::dominoe_player::DominoePlayer;
use crate::dominoe_deck::DominoeDeck;
use crate::dominoe::Dominoe;
/********
* dominoe_game has dominoe_deck and
* several dominoe_player's
* dominoe_game is responsible for ...
*   1. init a deck
*   2. init 4 players and player.init_hand(deck)
*   3. init spare-player ( has an empty spare train)
*   4. circulate through 4 player.take_turn()'s
*   5. check for player.hand_is_empty(): game over
*   6. check for deck.is_empty() and all player.token_up()'s: game over
*********/

#[derive(Debug)]
pub struct DominoeGame {
    deck: DominoeDeck,
    players: [DominoePlayer;4],
    spare_player: DominoePlayer,
    game_double:i32,
}

impl DominoeGame {
  pub fn new() -> Self {
	let mut mydeck= DominoeDeck::new();
	let mut player0=DominoePlayer::new();
	let mut player1=DominoePlayer::new();
	let mut player2=DominoePlayer::new();
	let mut player3=DominoePlayer::new();
	let mut myspare_player=DominoePlayer::new();
	myspare_player.set_spare();
	player0.init_hand(&mut mydeck);
	player1.init_hand(&mut mydeck);
	player2.init_hand(&mut mydeck);
	player3.init_hand(&mut mydeck);
	println!("DominoeGame constructor mydeck.show()");
	mydeck.show();
	println!("DominoeGame constructor after dealing mydeck.size(): {}",mydeck.size());
	println!("DominoeGame constructor after dealing player1.get_hand_length(): {}",player1.get_hand_length());
    let my_double = mydeck.pull_dominoe_double();
	DominoeGame {
	deck : mydeck,	
	players : [player0,player1,player2,player3],
	spare_player : myspare_player,
	game_double: my_double.unwrap_or(Dominoe::new(100,100)).get_head(),
    }
  }// end new()

  pub fn is_game_over(&self) -> bool {
	/*
	*   5. check for player.hand_is_empty(): game over
    *   6. check for deck.is_empty() and all player.token_up()'s: game over
    */
    for player in self.players.iter()
    {
	  if player.get_hand_length()==0
      {
	    return true;
      }
    }
    if self.deck.is_empty()
    {
	  for player in self.players.iter()
      {
	    if !player.is_train_token_up()
        {
	      return false;
        } 
      }
      return true;
	}
	return false; // should never hit here
  }// end is_game_over

  pub fn get_game_double(&self) -> i32 {
	return self.game_double;
  } // end get_game_double()

  pub fn play(&self) -> () {
	let mut play_num = 0;
	while self.is_game_over() != true
	{
		for player in self.players.iter()
		{
		  println!("player {} take_turn", play_num);	
		  while player.take_turn(self) == false
          {
	        println!("try again, player {} take_turn", play_num);
          }
          play_num += 1;
          play_num = play_num % 4;
		}
	}
  }// end play

}// end impl DominoeGame