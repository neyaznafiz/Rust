#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin{
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1)
  }
}


pub fn match_control() {
 
  value_in_cents(Coin::Quarter(UsState::Alaska));

  // ------
  let five = Some(5);
  println!("result: {:?}", plus_one(five));
  // // ---------------------------------

  let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => (),
  }

  fn add_fancy_hat() {}
  fn remove_fancy_hat() {}

  // ----------------
  let x: i8 = 5;
  let y: Option<i8> =Some(7);

  let sum  = x + y.unwrap_or( 0 );
  println!("sum: {:?}", sum);

  // -------- if let --------
  let config_max = Some(3);
    if let Some(max) = config_max { 
      println!("The maximum is configured to be {}", max)
  }

  // -
  let coin = Coin::Penny;
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
      println!("State quarter from {:?}!", state);
  } else {
     let result = count += 1;
     println!("result: {:?}", result);
  }
}