fn main() {
    
    println!("Prime numbers : {:?}",find_factors(input_terminal()));

}

////////////////////////////////////////////
/// TERMINAL INPUT
////////////////////////////////////////////

fn input_terminal() -> u32
{
    
    use std::io::{stdin,stdout,Write};

    let mut user_input = String::new();

    print!("Please enter a number from 2 to 200 : ");

    let _=stdout().flush();

    stdin().read_line(&mut user_input).expect("Did not enter a correct number");

    let _my_int:u32 = user_input.trim().parse().unwrap();

    return _my_int;
}

////////////////////////////////////////////
/// Erathostene sieve
////////////////////////////////////////////
fn find_factors(limit: u32) -> Vec<u32>
{

    let mut v: Vec<u32> = (2..limit).collect(); //·Generate·array·range$

    for i in 2..limit 
    {
        v.retain(|&x| x <= i || x % i != 0);
    }

    v
}
