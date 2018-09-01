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
}
