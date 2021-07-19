struct Player {
    name: String,
    age: u16,
}

enum Dimmer {
    On(u8),
    Off,
}

enum GameState {
    Paused,
    Running {
        player: Player,
        current_score: usize,
        dimmer: Dimmer,
    },
    GameOver(usize),
}

fn start_game(name: &str) -> GameState {
    GameState::Running {
        player: Player { name: name.to_string(), age: 25 },
        current_score: 0,
        dimmer: Dimmer::Off,
    }
}

fn loop_tick() {
    // Instance of an enum
    let mut state = GameState::Paused;

    state = start_game("bob");

    state = GameState::GameOver(100);
}

enum Output {
    NoOutput,
    Weak,
    Normal,
    Strong,
}

fn main() {
    let val = true;

    let outcome = match val {
        true => 1u8,
        false => 0,
    };

    match outcome {
        0 => {
            eprintln!("this was zero");
            eprintln!("Hello world");
        }
        value @ 1..=100 => {
            eprintln!("This was not zero, this was: {}", outcome);
            return
        },
        _ => println!("Everything is wrong, omg help")
        // 101..=254 => println!("Everything is wrong, omg help")
    }

    let dimmer = Dimmer::On(20);

    let output = match dimmer {
        Dimmer::On(0..=20) => Output::Weak,
        Dimmer::On(21..=150) => Output::Normal,
        Dimmer::On(_) => Output::Strong,
        Dimmer::Off => Output::NoOutput,
    };

    eprintln!("{:?}", "everything is amazing");

}
