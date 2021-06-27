

#[derive(Debug)]
pub struct Dominoe {
    lo: i32,
    hi: i32,
}

impl Dominoe {
  pub fn new(lo: i32, hi: i32) -> Dominoe {
    Dominoe { lo: lo, hi: hi, }
  }

  pub fn show(&self) -> ()  {
    println!("lo {0}, hi {1}",self.lo,self.hi);
  }
}


