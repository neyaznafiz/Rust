pub fn vector() {
  // scope one
  {  
    let _v = vec!["1", "2", "3"];
    let mut v2: Vec<u32> = Vec::new();

    let mut n = 0;

    loop{
      if n > 10 {
        break;
      } else{
        v2.push(n);
        n = n + 1;
        println!("{:?}", v2);
      }
    }

    // get vector value by index
    let _third: &u32 = &v2[0];

    // get vector value by get method
    let third_get: Option<&u32> =v2.get(55);
    match third_get {
      Some(third_get) => println!("The third element is : {:?}", third_get),
      None => println!("There is no third element"),
    }
    println!("{:?}, {:?}, {}", _v, v2, _third);
  }

  // scope two
  {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
      *i += 50;
    }

    for i in v {
        println!("{i}");
    }
  }
  
  // scope three
  {
    #[derive(Debug)]
    enum SpreadsheetCall {
      Int(i8),
      Float(f32),
      Text(String),
    }

    let row = vec![
      SpreadsheetCall::Int(3),
      SpreadsheetCall::Float(10.99),
      SpreadsheetCall::Text(String::from("vector test")),
    ];

    println!("{:?}", row);
  }

}