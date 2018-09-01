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

    'game: loop {
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

            while Dice::roll().is_doubles() {
                player.doubles_roll += 1;

                match player.doubles_roll {
                    1 => println!("You rolled doubles! Play again!"),
                    2 => println!("Doubles! If you do it again, you'll go to jail!"),
                    3 | _ => {
                        player.in_jail = true;
                        println!("Go to jail!");
                    }
                }
            }

            player.doubles_roll = 0;

            if !cli::press_enter() { break 'game; }
        }
    }
}
