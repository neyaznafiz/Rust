pub fn vector() {
  let _v = vec!["1", "2", "3"];
  let mut v2: Vec<u32> = Vec::new();

  v2.push(5);
  v2.push(10);
  v2.push(12);
  v2.push(2215);

  // get vector value by index
  let _third: &u32 = &v2[0];

  // get vector value by get method
  let third_get: Option<&u32> =v2.get(6);
  match third_get {
    Some(third_get) => println!("The third element is : {:?}", third_get),
    None => println!("There is no third element"),
  }

  println!("{:?}, {:?}, {}", _v, v2, _third);
}