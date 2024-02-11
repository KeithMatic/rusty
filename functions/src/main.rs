fn main() {
    println!(
        "{}",
        f({
            let x = 1;
            x + 2
        })
    );
}

fn f(x: i32) -> i32 {
    x + 1
}
