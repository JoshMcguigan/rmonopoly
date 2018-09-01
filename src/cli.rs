use std::io;

pub fn get_number_of_players() -> usize {
    loop {
        let mut number_of_players = String::new();
        io::stdin().read_line(&mut number_of_players)
            .expect("Failed to read stdin.");

        let number_of_players: usize = match number_of_players.trim().parse() {
            Ok(nop) => nop,
            Err(_) => {
                println!("Please enter an integer.");
                continue;
            },
        };
        if number_of_players == 0 {
            println!("You can't play and not play at the same time. This isn't Quantum Theory.");
            continue;
        }
        return number_of_players
    }
}

pub fn get_player_name(player_number: usize) -> String {
    let mut name = String::new();
    println!("Enter the name for player number {}.", player_number + 1);
    io::stdin().read_line(&mut name)
        .expect("Failed to read stdin.");

    name.trim().to_string()
}

pub fn printnls(newlines: u8) {
    for _ in 0..newlines {
        print!("\n");
    }
}
