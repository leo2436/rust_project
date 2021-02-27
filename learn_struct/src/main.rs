fn main() {
    struct User {
        name: String,
        age: u64,
        is_active: bool,
    }

    let person = User {
        age: 24,
        name: String::from("leo"),
        is_active: true,
    };

    let p2 = User {
        age: 32,
        name: String::from("john"),
        ..person
    };

    println!("age = {}, name = {}, is_active = {}", p2.age, p2.name, p2.is_active);
}
