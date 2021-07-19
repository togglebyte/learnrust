// -----------------------------------------------------------------------------
//     - Vanilla struct -
// -----------------------------------------------------------------------------
struct Computer {
    name: String,
    brand: String,
    cpu_count: u16,
    ram: usize,
}

impl Computer {
    fn new(a_name: String, cpu_count: u16, ram: usize) -> Self {
        Self {
            name: a_name, // we specify a_name here because a_name is not the same as name
            cpu_count,    // we don't have to specify cpu_count or ram, as the have the same
            ram,          // name as the args
            brand: "DodgyComputerCo".to_string(),
        }
    }

    // fn download_ram(computer: &mut Computer, extra_ram: usize) {
    //     computer.ram += extra_ram;
    // }

    // Mutable reference to self (self is an instance of a Computer / Self)
    fn download_ram(&mut self, extra_ram: usize) {
        self.ram += extra_ram;
    }

    // Reference to self
    fn print_name(&self) {
        eprintln!("{:?}", self.name);
    }

    fn destroy(self) {
        eprintln!("{:?}", "you can't use this computer anymore");
    }
}

// -----------------------------------------------------------------------------
//     - Tuple struct -
// -----------------------------------------------------------------------------
struct Name(String, String);

struct PlayerId(usize);

impl PlayerId {
    fn print(&self) {
        eprintln!("Player id: {}", self.0);
    }

    fn to_hp(self) -> Hitpoints {
        Hitpoints(self.0)
    }
}

struct Hitpoints(usize);

struct Player(PlayerId, Hitpoints);


fn print_player_id(id: PlayerId) {
    eprintln!("{}", id.0);
}

fn main() {
    
    let player = Player(
        PlayerId(12),
        Hitpoints(100),
    );

    let id = player.0;

    print_player_id(id);

    let vec = vec![1, 2, 3];

    let tup: (String, String) = ("Bob".to_string(), "Hoskins".to_string());
    tup.0;

    let name = Name("Bob".to_string(), "Hoskins".to_string());
    let Name(first, last) = name;
    eprintln!("{} {}", first, last);
    // tup[7];













    let mut comp = Computer::new("Bob".to_string(), 3, 32_000);
    // Computer::download_ram(&mut comp, 100);
    comp.download_ram(100);
    comp.destroy(); // comp is no more, memory marked as free
}
