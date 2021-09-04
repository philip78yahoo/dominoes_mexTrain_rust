extern crate rand;

use crate::dominoe::Dominoe;
use crate::dominoe_deck::DominoeDeck;
use crate::dominoe_hand::DominoeHand;

/********
* dominoe_game has dominoe_deck and several
* dominoe_player's
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
}

impl DominoeGame {
  pub fn new() -> Self {
	let mut mydeck=DominoeDeck::new();
	let mut player0=DominoePlayer::new();
	let mut player1=DominoePlayer::new();
	let mut player2=DominoePlayer::new();
	let mut player3=DominoePlayer::new();
	let mut spare_player=DominoePlayer::new();
	spare_player.set_spare();
	player0.init_hand(mydeck);
	player1.init_hand(mydeck);
	player2.init_hand(mydeck);
	player3.init_hand(mydeck);
	println!("DominoeGame constructor mydeck.show()");
	mydeck.show();
	println!("DominoeGame constructor after dealing mydeck.size(): {}",mydeck.size());
	println!("DominoeGame constructor after dealing player1.get_hand_length(): {}",player1.get_hand_length());

	DominoeGame {
	deck : mydeck,	
	players : [player0,player1,player2,player3],
    }
  }// end new()

  pub fn get_deck(&self)-> DominoeDeck {
     return self.deck;
  }// end get_deck()

  pub fn is_game_over(&self) -> bool {
	/*
	*   5. check for player.hand_is_empty(): game over
    *   6. check for deck.is_empty() and all player.token_up()'s: game over
    */
    for player in self.players
    {
	  if(player.get_hand_length()==0)
      {
	    return true;
      }
    }
    if(deck.is_empty())
    {
	  for player in self.players
      {
	    if(!player.is_train_token_up())
        {
	      return false;
        } 
      }
      return true;
	}
	return false; // should never hit here
  }// end is_game_over

  pub fn play(&self) -> () {
	let mut play_num = 0;
	while is_game_over() != true
	{
		for player in players
		{
		  while player.take_turn() == false
          {
	        println!("player {} makes a take_turn", play_num);
          }
          play_num += 1;
		}
	}
  }// end play

}// end impl DominoeGame