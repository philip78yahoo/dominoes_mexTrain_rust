extern crate rand;

use crate::dominoe::Dominoe;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct DominoeTrain {
    hand: VecDeque<Dominoe>,
    token: bool,
    game_double: Option<i32>,
}

impl DominoeTrain
{
  pub fn new() -> Self 
   {
	let myhand=VecDeque::new();
	let mytoken=false;
	let mygame_double=None;

	DominoeTrain 
	{
	 hand : myhand,	
     token : mytoken,
     game_double : mygame_double,
    }
   }// end new()
	

  pub fn size(&self) -> i32 { return self.hand.len() as i32; }

  pub fn add(&mut self, dominoe:Dominoe) -> bool { 
     // TODO access random element with index

     // vector.pop returns Option: Some<Dominoe> or None


     // if tail None than dominoe head must equal game-double
     // if tail Some then dominoe head must equal train tail
     if self.game_double.is_none()
     {
	   println!("game_double is none bad game init, returning false");
       return false;
     }
     else if self.hand.back().is_none() && dominoe.get_head()!= self.game_double.unwrap_or(100)
     {
       println!("dominoetail is none dominoe head does not match game_double, try orientation or bad dominoe");
       return false	;   
     }
     else if self.hand.back().unwrap_or(&Dominoe::new(100,100)).get_tail() != dominoe.get_head()
     {
	   println!("{} days", 31);
       println!("dominoe head != to train dominoetail's tail, try re-orient dominoe or bad dominoe");
       return false;	   
     }
     
     self.hand.push_back(dominoe);
     true 
  }// end add()

  pub fn show(&self) -> ()  {
     for dominoe in self.hand.iter() 
	  {dominoe.show()} 
   }// end show()

  pub fn up_token(&mut self) -> () {
	self.token = true
    
  }// end up_token

  pub fn down_token(&mut self)  {
	self.token = false
    
  }// end down_token

  pub fn is_token_up(&self) -> bool {
	self.token
    
  }// end up_token

  pub fn set_game_double(&mut self, the_game_double:i32) -> (){
	self.game_double=Some(the_game_double)
  }// end set_game_doube()

  pub fn get_tail(&self) -> i32 {
	//return self.dominoetail.as_ref().unwrap_or(&Dominoe::new(100,100)).get_tail();
	return self.hand.back().unwrap_or(&Dominoe::new(100,100)).get_tail()
  }// end set_game_doube()

  

}// end impl DominoeTrain