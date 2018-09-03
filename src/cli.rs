use std::io;

pub fn get_number_of_players() -> usize {
    println!("How many players?");
    loop {
        match user_input().trim().parse() {
            Ok(0) => {
                println!("You can't play and not play at the same time. This isn't Quantum Theory.");
            },
            Ok(nop) => return nop,
            Err(_) => {
                println!("Please enter an integer.");
            },
        };
    }
}

pub fn get_player_name(player_number: usize) -> String {
    println!("Enter the name for player number {}.", player_number + 1);
    user_input().trim().to_string()
}

pub fn printnls(newlines: u8) {
    for _ in 0..newlines {
        print!("\n");
    }
}

pub fn press_enter() -> bool {
    println!("Press ENTER or RETURN to continue.");
    println!("Press a key before ENTER or RETURN to quit.");

    let quit_input = user_input();

    quit_input.starts_with('\n')
}

fn user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read stdin.");

    user_input
}
