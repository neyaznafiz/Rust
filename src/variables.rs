pub fn variable(){
  // immutable variable
  let a = 5;

  // mutable variable
  let mut x = 5;
  println!("The value of x is: {}, {}", x, a);
  x = 6;
  println!("The value of x is: {}, {}", x, a);

  // constant variable
  const THIS_IS_CONST: u32 = 60 * 60 * 3;
  println!("The constant result is: {THIS_IS_CONST}");

  // shadowing
  let v = 1;
    println!("From first variable: {v}");

  {
    let v = v + 2;
      println!("From inner scope: {v}");

      let v = v * 2;
      println!("Final result from inner scope: {v}");
  }

  let v = v + 2;
    println!("From outside the scope: {v}");

  let v = v * 2;
    println!("Final result from outside the scope: {v}");
}