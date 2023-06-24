/*
To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
*/


struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
 }

pub fn discuss_struct() {

 let user_one = User {
  active: true,
  username: String::from("username123"),
  email: String::from("user@gmail.com"),
  sign_in_count: 1,
 };

 let user_two = User {
  email: String::from("user-tow@gmail.com"),
  username: String::from("user-two123"),
  ..user_one
 };

}

fn build_user(email: String, username: String) -> User {
  User {
      active: true,
      username,
      email,
      sign_in_count: 1,
  }
}