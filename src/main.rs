use ffxiv_rrc::calc;

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
        for roll in 1..=99 {
            let row: Vec<String> = (1..=24).map(|ps| format!("{}", calc(ps, roll))).collect();
            println!("{}", row.join(";"))
        }
    }
}
