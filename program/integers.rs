fn main() {
   let result = 10; //i32 by default
   let age:u32=20;
   let sum:i32=5-15;
   println!("result is {}",result);
   println!("sum is {} and age is {}", sum,age);

   let small_number1 = 10u8;
   println!("small number is {}",small_number1);
   let small_number2 = 10_u8; // This is easier to read
   let big_number = 100_000_000_i32; // 100 million is easy to read with _
   println!("{}, {}", small_number2, big_number);
   
   let number1 = 0________u8;
   let number2 = 1___6______2____4______i32;
   println!("{}, {}", number1, number2);
}
