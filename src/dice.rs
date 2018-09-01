use rand;
use rand::Rng;

pub struct Dice {
    values: (u8, u8)
}

impl Dice {
    pub fn roll() -> Self {
        let first = roll_die();
        let second = roll_die();

        println!("First dice is: {}", first);
        println!("Second dice is: {}", second);

        Dice {
            values: (first, second),
        }
    }

    pub fn is_doubles(&self) -> bool {
        self.values.0 == self.values.1
    }
}

fn roll_die() -> u8 {
    rand::thread_rng().gen_range(1, 7)
}
