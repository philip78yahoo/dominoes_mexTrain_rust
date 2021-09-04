extern crate rand;

use crate::dominoe::Dominoe;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct DominoeHand {
    hand: VecDeque<Dominoe>,
}

impl DominoeHand {
  pub fn new() -> Self {
	let myhand=VecDeque::new();
	DominoeHand {
	hand : myhand,	
    }
  }// end new()
	

  pub fn size(&self) -> i32 { self.hand.len() as i32 }

  pub fn pull_dominoe(&mut self) -> Option<Dominoe> { 
	 // TODO add case length is 0 and return None
     // TODO access random element with index

     // vector.pop returns Option: Some<Dominoe> or None
     return self.hand.pop_front(); 
  }// end pullDominoe()

  pub fn show(&self) -> ()  {
    for dominoe in &self.hand {
	 dominoe.show();
    }
  }// end show

  pub fn add(&mut self, dominoe: Dominoe) -> ()  {
      self.hand.push_back(dominoe);
  }// end add

}// end impl DominoeHand