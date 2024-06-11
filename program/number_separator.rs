fn main(){
    // For easy readability of large numbers, we can use a visual separator(_) underscore
    // to separate the digits, it will have no effect on the code or variable.
    let small_number = 10_u8; // This is easier to read
    let big_number = 100_000_000_i32; // 100 million is easy to read with _
    println!("{}, {}", small_number, big_number);
   
    let number1 = 0________u8;
    let number2 = 1___6______2____4______i32;
    println!("{}, {}", number1, number2);
}
