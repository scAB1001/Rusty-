#[derive(Debug)]
#[allow(dead_code)] // disable `dead_code` which warn against unused module
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("\n\tStandard:\n{:?}", peter);
    println!("\n\tPretty:\n{:#?}", peter);
}