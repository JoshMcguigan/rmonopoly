extern crate rand;

use rand::Rng;
use std::io;

mod player;
use player::Player;

mod cli;

fn jail_roll(jailed_player: &mut Player) -> bool {
    println!("{} is in jail", jailed_player.name);
    jailed_player.jail_count += 1;

    let first_dice = rand::thread_rng().gen_range(1, 7);
    let second_dice = rand::thread_rng().gen_range(1, 7);

    println!("First dice is: {}", first_dice);
    println!("Second dice is: {}", second_dice);

    if first_dice == second_dice {
        println!("{} got out of jail!", jailed_player.name);
        jailed_player.jail_count = 0;
        return false;
    }
    if jailed_player.jail_count == 3 {
        println!("You must pay $50 to get out now.");
        jailed_player.jail_count = 0;
        return false;
    }
    true
}

fn press_enter() -> bool {
    println!("Press ENTER or RETURN to continue.");
    println!("Press a key before ENTER or RETURN to quit.");

    let mut quit_input = String::new();
    io::stdin().read_line(&mut quit_input)
    .expect("Failed to read line.");

    if quit_input.starts_with('\n') {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    println!("How many players?");
    let number_of_players = cli::get_number_of_players();

    let mut player: Vec<Player> = Vec::new();
    for i in 0..number_of_players {
        let player_name = cli::get_player_name(i);
        player.push(Player::new(player_name));
    }
    let mut turn: usize = 0;
    loop {
        loop {
            if turn >= number_of_players {
                turn = 0;
            }
            if player[turn].in_jail {
                cli::printnls(100);

                if !jail_roll(&mut player[turn]) {
                    player[turn].in_jail = false;
                }
                turn += 1;
                break;
            }
            let first_dice = rand::thread_rng().gen_range(1, 7);
            let second_dice = rand::thread_rng().gen_range(1, 7);

            cli::printnls(100);

            println!("{}, it's your turn.", player[turn].name);

            println!("First dice is: {}", first_dice);
            println!("Second dice is: {}", second_dice);

            if first_dice != second_dice {
                player[turn].doubles_roll = 0;
                turn += 1;
                break;
            }
            else {
                player[turn].doubles_roll += 1;

                if player[turn].doubles_roll < 3 {
                    println!("You rolled doubles! Play again!");

                    if player[turn].doubles_roll == 2 {
                        println!("If you roll doubles again, you'll go to jail!");
                    }
                }
                if player[turn].doubles_roll >= 3 {
                    player[turn].in_jail = true;
                    player[turn].doubles_roll = 0;
                    println!("Go to jail!");
                    turn += 1;
                    break;
                }
            }
        }
        if press_enter() {
            continue;
        }
        else {
            break;
        }
    }
}
