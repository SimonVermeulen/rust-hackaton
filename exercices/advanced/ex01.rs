
fn print_counter(counter: &u32) {
    println!("Counter value: {}", counter);
}

fn increment_counter(counter: &mut u32) {
    *counter += 1;
}

fn main() {
    let mut counter = 0u32;

    while counter < 5 {
        print_counter(&counter);
        increment_counter(&mut counter);
    }
}
  