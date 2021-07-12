use std::collections::HashMap;

pub struct Symbols {
    next_free_address: u16,
    table: HashMap<String, u16>,
}

impl Symbols {
    pub fn new() -> Symbols {
        let mut table = HashMap::new();

        table.insert(String::from("R0"), 0);
        table.insert(String::from("R1"), 1);
        table.insert(String::from("R2"), 2);
        table.insert(String::from("R3"), 3);
        table.insert(String::from("R4"), 4);
        table.insert(String::from("R5"), 5);
        table.insert(String::from("R6"), 6);
        table.insert(String::from("R7"), 7);
        table.insert(String::from("R8"), 8);
        table.insert(String::from("R9"), 9);
        table.insert(String::from("R10"), 10);
        table.insert(String::from("R11"), 11);
        table.insert(String::from("R12"), 12);
        table.insert(String::from("R13"), 13);
        table.insert(String::from("R14"), 14);
        table.insert(String::from("R15"), 15);
        table.insert(String::from("SCREEN"), 16384);
        table.insert(String::from("KBD"), 24576);
        table.insert(String::from("SP"), 0);
        table.insert(String::from("LCL"), 1);
        table.insert(String::from("ARG"), 2);
        table.insert(String::from("THIS"), 3);
        table.insert(String::from("THAT"), 4);

        Symbols {
            next_free_address: 15,
            table,
        }
    }

    pub fn get_address(&mut self, symbol: String) -> u16 {
        match self.table.get(&symbol) {
            Some(&address) => address,
            None => {
                let address = self.add_variable(symbol);
                address
            }
        }
    }

    fn add_variable(&mut self, symbol: String) -> u16 {
        self.next_free_address += 1;
        self.table.insert(symbol, self.next_free_address);
        self.next_free_address
    }

    pub fn add_jump_symbol(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_address() {
        let mut symbols = Symbols::new();
        assert_eq!(symbols.get_address(String::from("R0")), 0);
        assert_eq!(symbols.get_address(String::from("R8")), 8);
        assert_eq!(symbols.get_address(String::from("R15")), 15);
        assert_eq!(symbols.get_address(String::from("SCREEN")), 16384);
        assert_eq!(symbols.get_address(String::from("KBD")), 24576);
        assert_eq!(symbols.get_address(String::from("SP")), 0);
        assert_eq!(symbols.get_address(String::from("LCL")), 1);
        assert_eq!(symbols.get_address(String::from("ARG")), 2);
        assert_eq!(symbols.get_address(String::from("THIS")), 3);
        assert_eq!(symbols.get_address(String::from("THAT")), 4);
    }

    #[test]
    fn new_symbols() {
        let mut symbols = Symbols::new();
        assert_eq!(symbols.get_address(String::from("FOO")), 16);
        assert_eq!(symbols.get_address(String::from("BAR")), 17);
        assert_eq!(symbols.get_address(String::from("BAZ")), 18);
    }

    #[test]
    fn add_jump_symbol() {
        let mut symbols = Symbols::new();
        symbols.add_jump_symbol(String::from("FOO"), 42);
        assert_eq!(symbols.get_address(String::from("FOO")), 42);
    }
}
