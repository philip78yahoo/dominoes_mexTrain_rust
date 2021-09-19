use crate::dominoe_game::DominoeGame;
use crate::dominoe_hand::DominoeHand;
use crate::dominoe_train::DominoeTrain;
use crate::dominoe::Dominoe;
use crate::dominoe_deck::DominoeDeck;
use std::io;
use std::io::*;

#[derive(Debug)]
pub struct DominoePlayer {
    train: DominoeTrain,
    hand: DominoeHand,
    id: i32 ,
    spare:bool,
}

impl DominoePlayer {
  pub fn new() -> Self {
	let myhand=DominoeHand::new();
	let mytrain=DominoeTrain::new();
	let myspare=false;
	let myid=0;
	
	DominoePlayer
	{
	  train: mytrain,
      hand: myhand,
      id: myid ,
      spare:myspare,
	}
  }// end new()

  pub fn set_spare(&mut self)->(){
	self.spare = true
  }// end set_spare

  pub fn get_spare(&self)->bool{
	return self.spare
  }// end get_spare

  pub fn init_hand(&mut self, deck:&mut DominoeDeck) -> () {
	for _cnt in 0..10 {
		//println!("going to push {}/{}",lo,hi);
		//the Dominoe(20,20) will stand out if problem in debug
	    self.hand.add(deck.pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	}// end cnt
  }// end init_hand	

  // need game to get access to ...
  // 1. deck if cant add domino, need to pull from deck
  // 2. other players hands to add to if other player's token is up
  // !!! this should not be public a player can take his own
  //     turn but not another player's turn
  // take_turn will ask the human what to do
  pub fn take_turn(&self, game:&DominoeGame)-> bool {
     println!("player.take_turn() not developed yet");

     self.hand.show();
     println!("game double {}",game.get_game_double());

     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("error: unable to read user input");
     println!("you entered {}",input);

     return true; 
  }// end take_turn()

  pub fn get_train_tail(&self)-> i32 {
     return self.train.get_tail()
  }// end get_train_tail()

  pub fn is_train_token_up(&self)-> bool {
     return self.train.is_token_up();
  }// end is_train_token_up()

  pub fn get_hand_length(&self)-> i32 {
     return self.hand.size()
  }// end get_hand_length()

}// end impl DominoePlayer
