fn main() {

    println!("Welcome to the FizzBuzz test in Rust");
    println!("------------------------------------");
    
    let mut i = 0;
    
    while i <= 100 {
    
        let mut message = String::from(i.to_string() + " ");
    
        if i % 3 == 0 {
            
            message.push_str("Fizz");
        }
        
        if i % 5 == 0 {
            
            message.push_str("Buzz");
        }
        
        println!("{}", message);
        
        i += 1;
    }
    
    println!("------------------------------------");
    println!("Finish");
}
