
use std::env;

use anyhow::Result;


// Every alib command must 
// implement this trait (see 
// AHelper in ahelper.rs)
pub trait Command: Sized {
    fn parse(args: &[String]) -> Result<Self>;
    fn go(self) -> Result<()>;
}


pub fn main<C>() -> Result<()>
where
    C: Command,
{
    let args: Vec<String> = env::args().collect();
    C::parse(&args)?.go()
}
