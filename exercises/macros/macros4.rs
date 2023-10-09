// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


/**
 * You only need to add a single character to make this compile.
The way macros are written, it wants to see something between each
"macro arm", so it can separate them.

That's all the macro exercises we have in here, but it's barely even
scratching the surface of what you can do with Rust's macros. For a more
thorough introduction, you can have a read through the little book of Rust
macros: https://veykril.github.io/tlborm/
 */


#[rustfmt::skip]
macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    () => {
        println!("Check out my macro!");
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
