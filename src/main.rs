// Importing library which will allow to read ints from strings
use std::str::FromStr;
// Importing library which will allow us to parse arguments from the command line
use std::env;

fn main(){
    println!("hallo");
    // Creating a vector which will hold all the args passed in the command line
    let mut numbers: Vec<u64> = Vec::new(); 

    // Prasing all the arguments from the command line
    for arg in env::args().skip(1){
        // Cast string to integer and check if was succesfull
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    // Check if the list is empty
    if numbers.len() == 0{
        eprint!("We don't have any numbers man");
        // If this occurs we must exit the program therefore we will use
        // the standard exit()
        std::process::exit(1);
    }

    // Now we are going to check what the GCD is
    // We are iterating over the array and store the value in the fist entry
    // This entry tell us which process is run
    let mut d: u64 = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }

    println!("The greatests common divisor of {:?} is {}", numbers , d);

    
}


fn gcd(mut n: u64 , mut m: u64) -> u64 {
    // Assert the fucker is zero
    assert!( m != 0 && n != 0);
    while m != 0{
        if m < n {
            let t = n;
            n = m;
            m = t;  
        } 
        m = m % n;        
    }
    // Returning ans
    n 
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(6,12),6);
}
