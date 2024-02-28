use protocol::macros::EnumToString;


#[derive(EnumToString)]
enum Test {
    Ok,
    Error,
}


fn main() {
    println!("Hello, world!");
    println!("{}", Test::Ok.enum_to_string());
}

