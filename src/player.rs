use dice::Dice;

pub struct Player {
    pub name: String,
    pub jail_count: u8,
    pub in_jail: bool,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            jail_count: 0,
            in_jail: false,
        }
    }

    pub fn take_turn(&mut self) {
        let mut doubles_roll = 0u8;

        if self.in_jail {
            self.jail_roll();
            return;
        } else {
            println!("{}, it's your turn.", self.name);

            while Dice::roll().is_doubles() {
                doubles_roll += 1;

                match doubles_roll {
                    1 => println!("You rolled doubles! Play again!"),
                    2 => println!("Doubles! If you do it again, you'll go to jail!"),
                    3 | _ => {
                        self.in_jail = true;
                        println!("Go to jail!");
                    }
                }
            }
        }
    }

    fn jail_roll(self: &mut Player) {
        println!("{} is in jail", self.name);
        self.jail_count += 1;

        let dice = Dice::roll();

        if dice.is_doubles() {
            println!("{} got out of jail!", self.name);
            self.get_out_of_jail();
        }
        if self.jail_count == 3 {
            println!("You must pay $50 to get out now.");
            self.get_out_of_jail();
        }
    }

    fn get_out_of_jail(&mut self) {
        self.jail_count = 0;
        self.in_jail = false;
    }
}
