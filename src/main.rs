

mod itemtype;
mod utility;


use itemtype::item_type;
use utility::Utility;
use std::io::prelude::*;
use std::fs::File;
use std::time;
use std::collections::VecDeque;


fn main()
{
    let a = time::SystemTime::now();
    let mut inFile = File::open("input.eu4").unwrap(); // how check error?
    let mut result = VecDeque::new();

    ::utility::Utility::reserve(&mut inFile, &mut result);     

    let b = time::SystemTime::now();
    let c = b.duration_since(a);


   // println!("{:?}", result);
    println!("{:?}", c);
}

