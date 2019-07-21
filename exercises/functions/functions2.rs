// functions2.rs
// Make me compile! Scroll down for hints :)

fn main() {
    call_me(3);
}

<<<<<<< HEAD
fn call_me(num:i32) {
=======
fn call_me(num) {
>>>>>>> 7f225fe26a757e4b8426e90a6fd96275107c507d
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}




























// Rust requires that all parts of a function's signature have type annotations,
// but `call_me` is missing the type annotation of `num`.
