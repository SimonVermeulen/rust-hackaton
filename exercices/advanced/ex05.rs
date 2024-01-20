fn main() {
    let closure = |x: &[i32]| -> (i32, i32) {(x[0], x[x.len() - 1])};
  
    println!("{:?}", closure(&[1, 2, 3, 4, 5, 6]));
}