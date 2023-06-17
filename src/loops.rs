pub fn loops(){
  /*
   we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support the keyboard shortcut ctrl-c to interrupt a program that is stuck in a continual loop.
  */
  loop {
    println!("again");
    // break;
  }


// Returning Values from Loops
let mut counter: i8 = 0;

    let result: i8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


// Loop Labels to Disambiguate Between Multiple Loops
let mut count: i8 = 0;

'counting_up: loop{
  println!("count = {count}");

  let mut remaining: i8 = 10;
  loop{
    println!("remaining = {remaining}");
    if remaining == 3 {
    break;
    }
    if count == 5 {
      break 'counting_up;
    }
    remaining -= 1;
  }
  count += 1;
}
println!("End count = {count}");


// Conditional Loops with while
 let mut number : i8 = 3;

 while number != 0 {
  number -= 1;
 }

 println!("number = {number}");


// Looping Through a Collection with for
let a: [isize; 5] = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
  println!("the index value is : {}", a[index]);

  index += 1;
}

for element in a {
  println!("the value is: {}", element);
}

for number in (1..4).rev(){
  println!("{number}")
}
}