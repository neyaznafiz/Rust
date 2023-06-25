
mod variables;
mod conditions;
mod loops;
mod slice_type;
mod structs;
mod enums;
mod match_control;



fn main() {
    variables::variable();
    conditions::conditions();
    loops::loops();
    slice_type::slice();
    structs::discuss_struct();
    enums::enums();
    match_control::match_control();
}
