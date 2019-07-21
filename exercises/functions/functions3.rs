// functions3.rs
// Make me compile! Scroll down for hints :)

fn main() {
<<<<<<< HEAD
    call_me(5);
=======
    call_me();
>>>>>>> 7f225fe26a757e4b8426e90a6fd96275107c507d
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}




























// This time, the function *declaration* is okay, but there's something wrong
// with the place where we're calling the function.
