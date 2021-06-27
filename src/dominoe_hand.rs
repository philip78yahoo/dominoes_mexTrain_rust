extern crate rand;

use crate::dominoe::Dominoe;
use core::cell::Cell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct DominoeHand {
    hand: VecDeque<Dominoe>,
    length: Cell<i32>, // Cell mutable at field level
}

impl DominoeHand {
  pub fn new() -> Self {
	let myhand=VecDeque::new();
	DominoeHand {
	hand : myhand,	
    length: Cell::new(0),
    }
  }// end new()
	

  pub fn size(&self) -> i32 { return self.length.get(); }

  pub fn pull_dominoe(&mut self) -> Option<Dominoe> { 
	 // TODO add case length is 0 and return None
     // TODO access random element with index
     self.length.set(self.length.get() - 1);

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
      self.length.set(self.length.get() + 1);
  }// end add

}// end impl DominoeHand