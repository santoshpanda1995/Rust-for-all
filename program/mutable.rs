fn main(){
  // Variables are immutable by default, i.e. the variable's value cannot be changed
  // once a value is bound to a var_name. TO make it mutable, we have to prefix the 
  // variable name with (mut) keyword. Then the value can be chanegd.

  let mut fees=25_000;
  println!("fees is {}",fees);
  fees=35_000;
  println!("fees changed to {}",fees);
}
