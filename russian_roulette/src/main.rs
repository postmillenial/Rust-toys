use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    println!("Please enter your guess:");

    let mut guess = String::new();
    let mut name = String::new();
 
    io::stdin().read_line(&mut guess);
    println!("I should enter a question:");
    io::stdin().read_line(&mut name);

    //still don't quite get borrowing + mutability.
    let chartrim: &[char] = &['\n'];

    let trimmed_str: &str = name.trim_matches(chartrim);
    // i get that i have to create a &str type for the trim_matches return value
    // and i understand that here, the mutable name gets borrowed as immutable in order to call trim_matches
    
    println!("{}?!?!?!?!?", trimmed_str);
    //and as a result of that borrow  i can't do
    //io::stdin().read_line(&mut name);
    //I have to make a new var instead, but I'm not sure why. Is there an alternate way to end the borrow?
    let mut actual_name_var = String::new();
    io::stdin().read_line(&mut actual_name_var );
    let trimmed_name: &str = actual_name_var.trim_matches(chartrim);
    println!("Fuck! it's you, {}! And you guessed {}", trimmed_name, guess);

}
