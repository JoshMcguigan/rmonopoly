extern crate rand;

mod player;
use player::Player;

mod cli;

mod dice;
use dice::Dice;

fn jail_roll(jailed_player: &mut Player) -> bool {
    println!("{} is in jail", jailed_player.name);
    jailed_player.jail_count += 1;

    let dice = Dice::roll();

    if dice.is_doubles() {
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

fn main() {
    let number_of_players = cli::get_number_of_players();

    let mut players: Vec<Player> = Vec::new();
    for i in 0..number_of_players {
        let player_name = cli::get_player_name(i);
        players.push(Player::new(player_name));
    }
    loop {
        for mut player in players.iter_mut() {
            if player.in_jail {
                cli::printnls(100);

                if !jail_roll(player) {
                    player.in_jail = false;
                }
                continue;
            }

            cli::printnls(100);

            println!("{}, it's your turn.", player.name);

            let dice = Dice::roll();

            if !dice.is_doubles() {
                player.doubles_roll = 0;
                continue;
            }
            else {
                player.doubles_roll += 1;

                if player.doubles_roll < 3 {
                    println!("You rolled doubles! Play again!");

                    if player.doubles_roll == 2 {
                        println!("If you roll doubles again, you'll go to jail!");
                    }
                }
                if player.doubles_roll >= 3 {
                    player.in_jail = true;
                    player.doubles_roll = 0;
                    println!("Go to jail!");
                    continue;
                }
            }
        }
        if cli::press_enter() {
            continue;
        }
        else {
            break;
        }
    }
}
