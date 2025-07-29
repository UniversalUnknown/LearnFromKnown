//Calculator
use inquire::{InquireError, Select};
fn main() -> Result<(), InquireError> {
    println!("Test 2");
    //Value of A
    println!("Enter A value: ");
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed");
    let num1: u32 = a.trim().parse().expect("Not a valid number");
    //Value of B
    println!("Enter B vlaue: ");
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).expect("Failed");
    let num2: u32 = b.trim().parse().expect("Not a valid number");
    //Choice option
    let options = vec!["Addition", "Multiplication", "Subraction", "Division"];
    let selected_category = Select::new("Select an operation:", options).prompt()?;
    let ret: u32;
    match selected_category {
        "Addition" => ret = add(num1, num2),
        "Multiplication" => ret = mult(num1, num2),
        "Subraction" => ret = sub(num1, num2),
        "Division" => ret = div(num1, num2),
        _ => return Err(InquireError::from(Box::from("Invalid selection"))),
    }
    println!("The {selected_category} is: {ret}");
    Ok(())
}
//function
fn mult(a: u32, b: u32) -> u32 {
    return a * b;
}
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
fn sub(a: u32, b: u32) -> u32 {
    return a - b;
}
fn div(a: u32, b: u32) -> u32 {
    return a / b;
}
