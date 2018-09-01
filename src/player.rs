use dice::Dice;
use cli;

pub struct Player {
    pub name: String,
    pub doubles_roll: u8,
    pub jail_count: u8,
    pub in_jail: bool,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            doubles_roll: 0,
            jail_count: 0,
            in_jail: false,
        }
    }

    pub fn take_turn(&mut self) {
        if self.in_jail {
            cli::printnls(100);

            if !self.jail_roll() {
                self.in_jail = false;
            }
            return;
        }

        cli::printnls(100);

        println!("{}, it's your turn.", self.name);

        while Dice::roll().is_doubles() {
            self.doubles_roll += 1;

            match self.doubles_roll {
                1 => println!("You rolled doubles! Play again!"),
                2 => println!("Doubles! If you do it again, you'll go to jail!"),
                3 | _ => {
                    self.in_jail = true;
                    println!("Go to jail!");
                }
            }
        }

        self.doubles_roll = 0;
    }

    fn jail_roll(self: &mut Player) -> bool {
        println!("{} is in jail", self.name);
        self.jail_count += 1;

        let dice = Dice::roll();

        if dice.is_doubles() {
            println!("{} got out of jail!", self.name);
            self.jail_count = 0;
            return false;
        }
        if self.jail_count == 3 {
            println!("You must pay $50 to get out now.");
            self.jail_count = 0;
            return false;
        }
        true
    }
}
