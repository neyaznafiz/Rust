#[derive(Debug)]
enum IpAddrKind{
  V4,
  V6,
}

#[derive(Debug)]
enum IpAddr{
  V4(String),
  V6(String),
}


pub fn enums() {
  let four = IpAddrKind::V4;
  // let six = IpAddrKind::V6;

  route(four );
  route(IpAddrKind::V6);

  let home = IpAddr::V4(String::from(":127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));

  println!("IpAddr: {:?}", home);
  println!("IpAddr: {:?}", loopback);
}

fn route(ip_kind: IpAddrKind){
  println!("IpAddrKind: {:?}", ip_kind)
}