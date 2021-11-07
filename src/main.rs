extern crate autopilot;
use autopilot::alert::alert;
use autopilot::mouse;

fn click(count: u32, delay: u64, left_not_right: bool) {
    for _ in 0..count {
        if left_not_right {
            mouse::click(mouse::Button::Left, Some(delay));
        } else {
            mouse::click(mouse::Button::Right, Some(delay));
        }
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rounds = args
        .get(1)
        .expect("Pass the number of rounds")
        .parse::<u64>()
        .expect("Pass an integer value");
    let clicks_per_round = args
        .get(2)
        .expect("Pass the number of clicks per round")
        .parse::<u32>()
        .expect("Pass an integer value");
    let delay_between_rounds = args
        .get(3)
        .expect("Pass the delay between rounds in milliseconds")
        .parse::<u64>()
        .expect("Pass an integer value");
    let delay_for_click = args
        .get(4)
        .expect("Pass the delay between clicks in milliseconds")
        .parse::<u64>()
        .expect("Pass an integer value");
    let left_not_right = args.get(5).map(|a| a != "right").unwrap_or(true);
    println!("Clicking {} times per each round of {} with a {} ms delay between rounds and a {} ms delay between clicks.", clicks_per_round, rounds, delay_between_rounds, delay_for_click);
    for _ in 0..rounds {
        click(clicks_per_round, delay_for_click, left_not_right);
        std::thread::sleep(std::time::Duration::from_millis(delay_between_rounds));
    }

    alert(
        &format!(
            "Clicking {} times per each round of {} with a {} ms delay between rounds.",
            clicks_per_round, rounds, delay_between_rounds
        ),
        Some("Done"),
        None,
        None,
    );
}
