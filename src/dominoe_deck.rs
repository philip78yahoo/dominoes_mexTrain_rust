extern crate rand;

use crate::dominoe::Dominoe;
use core::cell::Cell;
use rand::seq::SliceRandom; // 0.6.5

#[derive(Debug)]
pub struct DominoeDeck {
    deck: Vec<Dominoe>,
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

    let mut rng = rand::thread_rng();
    mydeck.shuffle(&mut rng);

	DominoeDeck {
	deck : mydeck,	
    }
  }// end new()
	

  pub fn size(&self) -> i32 { return self.deck.len() as i32; }

  pub fn pull_dominoe(&mut self) -> Option<Dominoe> { 
	 // TODO add case length is 0 and return None
 
     // vector.pop returns Option: Some<Dominoe> or None
     return self.deck.pop(); 
  }// end pullDominoe()

  pub fn pull_dominoe_double(&mut self) -> Option<Dominoe> { 
	 // TODO add case length is 0 and return None
 
    let mut i:usize =0;
    while i < self.deck.len() {
        if self.deck[i].is_double()
        {
	      return Some(self.deck.remove(i));
        }
        else
        {
	       i=i+1;
        }
    }// end while

     // vector.pop returns Option: Some<Dominoe> or None
     return None; 
  }// end pullDominoe()

  pub fn show(&self) -> ()  {
    for dominoe in &self.deck {
	 dominoe.show();
    }
  }// end show()

  pub fn is_empty(&self) -> bool  {
     self.deck.is_empty()
  }// end is_empty()

}// end impl DominoeDeck