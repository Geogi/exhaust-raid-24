fn main() {
    for participants in 1..=24 {
        let adversaries = participants - 1;
        let winning_tie_chance = winning_tie(adversaries);
        for roll in 1..=99 {
            let all_below_chance = all_below(adversaries, roll);
            let overall_chance = all_below_chance + winning_tie_chance;
            print!("{};", overall_chance);
        }
        println!()
    }

}

fn winning_tie(adversaries: i32) -> f64 {
    let mut chance = 0.0;
    for people_tied_with_me in 1..=adversaries {
        let chance_of_tie = f64::powi(1.0 / 99.0, people_tied_with_me)
            * f64::powi(98.0 / 99.0, adversaries - people_tied_with_me);
        let chance_of_winning = 1.0 / f64::from(people_tied_with_me + 1);
        chance += chance_of_tie * chance_of_winning;
    }
    chance
}

fn all_below(adversaries: i32, roll: i32) -> f64 {
    let target = f64::from(roll - 1);
    let one_below = target / 99.0;
    let all_below = f64::powi(one_below, adversaries);
    all_below
}
