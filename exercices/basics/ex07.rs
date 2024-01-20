fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut result: i32 = 0;

    for num in slice {
        result += num;
    }
    return result;
}

fn print_modulo(number1: i32, number2: i32) {
    println!("{}", number1 % number2);
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
}
  