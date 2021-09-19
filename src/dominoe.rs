

#[derive(Debug)]
pub struct Dominoe {
    lo: i32,
    hi: i32,
    orientation: bool,
}

impl Dominoe {
  pub fn new(lo: i32, hi: i32) -> Dominoe {
    Dominoe { lo: lo, hi: hi, orientation: true,}
  }// end new

 pub fn is_double(&self) -> bool {
	return self.hi==self.lo
  }// end is_double

 pub fn show(&self) -> ()  {
    println!("head {0}, tail {1}, orientation {2}",self.get_head(),self.get_tail(),self.orientation);
  }// end show

 pub fn get_head(&self) -> i32 {
	if self.orientation
	{ self.hi }
    else
    { self.lo }
  }// end getHead

 pub fn get_tail(&self) -> i32 {
	if self.orientation
	{ self.lo }
    else
    { self.hi }
  }// end getTail

 pub fn toggle_orientation(&mut self) -> () {
	self.orientation = !self.orientation;
  }// end toggleOrientation

}


