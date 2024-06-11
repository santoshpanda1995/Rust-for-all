fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    result // this is the i32 that we return
}

fn main() {
    let multiply_result = multiply(8, 9);
    println!("8 times 9 is {} ", multiply_result);
}
