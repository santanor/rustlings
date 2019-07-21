// variables3.rs
// Make me compile! Scroll down for hints :)

fn main() {
<<<<<<< HEAD
    let mut x = 3;
=======
    let x = 3;
>>>>>>> 7f225fe26a757e4b8426e90a6fd96275107c507d
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);
}































// In Rust, variable bindings are immutable by default. But here we're trying
// to reassign a different value to x! There's a keyword we can use to make
// a variable binding mutable instead.
