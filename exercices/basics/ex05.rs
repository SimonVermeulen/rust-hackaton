fn evaluate_number(number: i32)  {
    if number == 0 {
        println!("Zero");
    } else if number > 0 {
        println!("Positive");
    } else {
        println!("Negative");
    }
}

fn main() {
  evaluate_number(0);
  evaluate_number(5);
  evaluate_number(-5);
}