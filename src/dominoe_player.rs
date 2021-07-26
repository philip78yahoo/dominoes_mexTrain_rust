#[derive(Debug)]
pub struct DominoePlayer {
    train: DominoeTrain,
    hand: DominoeHand,
    id:<i32>,
    spare:bool,
}

impl DominoePlayer {
  pub fn new() -> Self {
	let mut hand=DominoeHand::new();
	let mut train=DominoeTrain::new();
	let mut spare=false;
  }// end new()

  pub fn set_spare()->(){
	spare = true;
  }// end set_spare

  pub fn get_spare()->bool{
	return spare;
  }// end get_spare

  pub fn init_hand(&self, &game:DominoeGame) -> () {
	for _cnt in 0..10 {
		//println!("going to push {}/{}",lo,hi);
		//the Dominoe(20,20) will stand out if problem in debug
	    self.hand.add(game.get_deck().pull_dominoe().unwrap_or(Dominoe::new(20,20)));	
	}// end cnt
  }// end init_hand	

  // need game to get access to ...
  // 1. deck if no move, need to pull
  // 2. other players hands to add to if their token is up
  // !!! this should not be public a player can make his own
  //     move but not another player's move
  // move will ask the human what to do
  fn move(&self, &game:DominoeGame)-> bool {
     println!("player.move() not developed yet");
     return true; 
  }// end move()

  pub fn get_train_tail(&self)-> DominoeTrain {
     return self.train.get_tail();
  }// end get_train()

  pub fn is_train_token_up(&self)-> bool {
     return self.train.is_token_up();
  }// end is_train_token_up()

  pub fn get_hand_length(&self)-> <i32> {
     return self.hand.size();
  }// end get_hand_length()
