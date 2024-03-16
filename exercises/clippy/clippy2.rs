// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.
// Execute `rustlings note clippy2` or use the `note clippy2` watch subcommand for lzz's note.



// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);
}
