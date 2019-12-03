pub fn calc(players: i32, roll: i32) -> f64 {
    let losing: f64 = (roll - 1).into();
    let adverse = players - 1;

    1.0 / 99.0_f64.powi(adverse)
        * (losing.powi(adverse)
            + (1..=adverse).fold(0.0, |acc, n| {
                acc + losing.powi(adverse - n) / f64::from(n + 1)
            }))
}
