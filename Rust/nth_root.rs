fn nth_root(x: f32, n: f32) -> f32 {
    let mut guess = x;
    let mut next;

    loop {
        next = (((n - 1.0) * guess) + (x / guess.powf(n - 1.0))) / n;

        if guess * guess == x {
            break;
        }

        println!("{},{}", next, guess); // Optional debug log
        guess = next;
    }

    guess
}

fn main() {
    let mut x: f32 = 27.0;
    println!("{}", nth_root(x, 3.0));
}
