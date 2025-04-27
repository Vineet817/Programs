fn reciprocal(x: f64) -> f64 {
    let mut guess: f64 = 0.00000001;
    for i in 0..1000 {
        let new_guess = guess * (2.0 - x * guess);
        println!("iteration {}: {}", i, guess);
        if new_guess == guess {
            return new_guess;

        }
        guess = new_guess;
    }
    guess
}

fn main() {
    let result = reciprocal(10.0);
    println!("reciprocal: {}", result);
}
