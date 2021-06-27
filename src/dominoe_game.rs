extern crate rand;

use crate::dominoe::Dominoe;
use crate::dominoe_deck::DominoeDeck;
use crate::dominoe_hand::DominoeHand;

/********
* dominoe_game has dominoe_deck and several
* dominoe_hand's
* dominoe_game is responsible for taking
* dominoe's from dominoe_deck and putting them
* in dominoe_hand's
*********/

#[derive(Debug)]
pub struct DominoeGame {
    deck: DominoeDeck,
    hands: [DominoeHand;4],
}

impl DominoeGame {
  pub fn new() -> Self {
	let mut mydeck=DominoeDeck::new();
	let mut myhand0=DominoeHand::new();
	let mut myhand1=DominoeHand::new();
	let mut myhand2=DominoeHand::new();
	let mut myhand3=DominoeHand::new();
	for _cnt in 0..10 {
		//println!("going to push {}/{}",lo,hi);
		//the Dominoe(20,20) will stand out if problem in debug
	    myhand0.add(mydeck.pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	    myhand1.add(mydeck.pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	    myhand2.add(mydeck.pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	    myhand3.add(mydeck.pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	}// end cnt
	println!("DominoeGame constructor mydeck.show()");
	mydeck.show();
	println!("DominoeGame constructor after dealing mydeck.size(): {}",mydeck.size());
	println!("DominoeGame constructor after dealing myhand1.size(): {}",myhand1.size());
	println!("DominoeGame constructor after dealing myhand1.show()");
	myhand0.show();

	DominoeGame {
	deck : mydeck,	
	hands : [myhand0,myhand1,myhand2,myhand3],
    }
  }// end new()

  //&mut self, dominoe: Dominoe
  pub fn pull_from_hand(&mut self, hand_num:usize) -> Dominoe {
     //Dominoe(20,20) obvious in debug something wrong
	 self.hands[hand_num].pull_dominoe().unwrap_or(Dominoe::new(20,20))
  }// end pull_from_hand


}// end impl DominoeGame