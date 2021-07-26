extern crate rand;

use crate::dominoe::Dominoe;
use core::cell::Cell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct DominoeTrain {
    hand: VecDeque<Dominoe>,
    tail: Option<Dominoe>,
    token: bool,
    game_double: Option<i32>,
}

impl DominoeTrain {
  pub fn new() -> Self {
	let myhand=VecDeque::new();
	DominoeHand {
	hand : myhand,
	game_double : None,	
    tail: None,
    token: false,
    }
  }// end new()
	

  pub fn size(&self) -> i32 { return self.length.get(); }

  pub fn add(&mut self, dominoe:Dominoe) -> bool { 
     // TODO access random element with index

     // vector.pop returns Option: Some<Dominoe> or None


     // if tail None than dominoe head must equal game-double
     // if tail Some then dominoe head must equal train tail
     if game_double.is_none()
     {
	   println("game_double is none bad game init, returning false");
       false
     }
     else if self.tail.is_none() && dominoe.get_head()!= game_double;
     {
       println("tail is none dominoe head does not match game_double, try orientation or bad dominoe");
       false	   
     }
     else if tail.get_tail() != dominoe.get_head()!= game_double;
     {
       println("dominoe head != to train tail's tail, try re-orient dominoe or bad dominoe");
       false	   
     }
     
     self.hand.push_back(dominoe);
     self.tail =  dominoe;
     true 
  }// end add()

  pub fn show(&self) -> ()  {
    for dominoe in &self.hand {
	 dominoe.show();
    }// end show()

  pub fn up_token(&self) -> () {
	self.token = true;
    
  }// end up_token

  pub fn down_token(&self) -> () {
	self.token = false;
    
  }// end up_token

  pub fn is_token_up(&self) -> () {
	self.token
    
  }// end up_token

  pub fn set_game_double(&self, the_game_double:i32) -> (){
	self.game_double=the_game_double;
  }// end set_game_doube()

  pub fn get_tail(&self) -> i32 {
	return self.tail.get_tail();
  }// end set_game_doube()

  

}// end impl DominoeTrain