fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // Because "if" is an expression, we can use it on the RHS of a "let" statement to assign the outcome to a variable, as in Listing 3-2.

    let condition = true;

    let number_one = if condition { 5 } else { 6 };
    println!("{number_one}");

    // let condition = gameOver ? "You lose!" : "Continue playing!"; JavaScript
}