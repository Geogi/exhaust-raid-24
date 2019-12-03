const DEFAULT_ROLL: i32 = 90;
const DEFAULT_PLAYERS: i32 = 8;

fn main() {
    let matches = clap::App::new("ffxiv-rrc")
        .arg(clap::Arg::with_name("ROLL"))
        .arg(clap::Arg::with_name("PLAYERS"))
        .get_matches();
    if let Some(roll) = matches.value_of("ROLL") {
        let roll: i32 = roll
            .parse()
            .map(|r: i32| {
                if r >= 1 && r <= 99 {
                    r
                } else {
                    eprintln!(
                        "ROLL arg out of bounds, assuming default of {}",
                        DEFAULT_ROLL
                    );
                    DEFAULT_ROLL
                }
            })
            .unwrap_or_else(|_| {
                eprintln!(
                    "Cannot parse ROLL arg, assuming default of {}.",
                    DEFAULT_ROLL
                );
                DEFAULT_ROLL
            });
        if let Some(players) = matches.value_of("PLAYERS") {
            let players: i32 = players
                .parse()
                .map(|p: i32| {
                    if p > 0 {
                        p
                    } else {
                        eprintln!(
                            "PLAYERS arg cannot be zero, assuming default of {}",
                            DEFAULT_PLAYERS
                        );
                        DEFAULT_PLAYERS
                    }
                })
                .unwrap_or_else(|_| {
                    eprintln!(
                        "Cannot parse PLAYERS arg, assuming default of {}.",
                        DEFAULT_PLAYERS
                    );
                    DEFAULT_PLAYERS
                });
            println!("{}", calc(players, roll));
        } else {
            println!("8-man raid: {:.2}%", 100.0 * calc(8, roll));
            println!("24-man raid: {:.2}%", 100.0 * calc(24, roll));
        }
    } else {
        table_mode()
    }
}

fn table_mode() {
    for roll in 1..=99 {
        let row: Vec<String> = (1..=24).map(|ps| format!("{}", calc(ps, roll))).collect();
        println!("{}", row.join(";"))
    }
}

fn calc(players: i32, roll: i32) -> f64 {
    let losing: f64 = (roll - 1).into();
    let adverse = players - 1;

    1.0 / 99.0_f64.powi(adverse)
        * (losing.powi(adverse)
            + (1..=adverse).fold(0.0, |acc, n| {
                acc + losing.powi(adverse - n) / f64::from(n + 1)
            }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_table_mode() {
        table_mode()
    }
}
