extern crate rand;

//not yet: use rand::{thread_rng, Rng};

use crate::dominoe::Dominoe;
use core::cell::Cell;


pub struct DominoeDeck {
    deck: Vec<Dominoe>,
    length: Cell<i32>, // Cell mutable at field level
}

impl DominoeDeck {
  pub fn new() -> Self {
	let mut mydeck=Vec::new();
	for lo in 0..13 {
	  for hi in lo..13{
		//println!("going to push {}/{}",lo,hi);
	    mydeck.push(Dominoe::new(lo,hi));	
	  }// end hi
	}// end lo
	DominoeDeck {
	deck : mydeck,	
    length: Cell::new(13*13),
    }
  }
	

  pub fn size(&self) -> i32 { return self.length.get(); }

  pub fn pull_dominoe(&mut self) -> Option<Dominoe> { 
	 // TODO add case length is 0 and return None
     self.length.set(self.length.get() - 1);

     // vector.pop returns Option: Some<Dominoe> or None
     return self.deck.pop(); 
  }// end pullDominoe()

  pub fn show(&self) -> ()  {
    for dominoe in &self.deck {
	 dominoe.show();
    }
  }// end show
}// end impl DominoeDeck