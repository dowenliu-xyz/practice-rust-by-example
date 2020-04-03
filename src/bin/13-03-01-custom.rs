#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}

// 1.
// rustc -o custom 13-03-01-custom.rs && ./custom

// 2.
// rustc -o custom --cfg some_condition 13-03-01-custom.rs && ./custom
