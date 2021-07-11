use std::collections::HashMap;

pub struct Symbols<'a> {
    next_free_address: u16,
    table: HashMap<&'a str, u16>,
}

impl<'a> Symbols<'a> {
    pub fn new() -> Symbols<'a> {
        let mut table = HashMap::new();

        table.insert("R0", 0);
        table.insert("R1", 1);
        table.insert("R2", 2);
        table.insert("R3", 3);
        table.insert("R4", 4);
        table.insert("R5", 5);
        table.insert("R6", 6);
        table.insert("R7", 7);
        table.insert("R8", 8);
        table.insert("R9", 9);
        table.insert("R10", 10);
        table.insert("R11", 11);
        table.insert("R12", 12);
        table.insert("R13", 13);
        table.insert("R14", 14);
        table.insert("R15", 15);
        table.insert("SCREEN", 16384);
        table.insert("KBD", 24576);
        table.insert("SP", 0);
        table.insert("LCL", 1);
        table.insert("ARG", 2);
        table.insert("THIS", 3);
        table.insert("THAT", 4);

        Symbols {
            next_free_address: 15,
            table,
        }
    }

    pub fn get_address(&mut self, symbol: &'a str) -> u16 {
        match self.table.get(symbol) {
            Some(&address) => address,
            None => {
                let address = self.add_variable(symbol);
                address
            }
        }
    }

    fn add_variable(&mut self, symbol: &'a str) -> u16 {
        self.next_free_address += 1;
        self.table.insert(symbol, self.next_free_address);
        self.next_free_address
    }

    pub fn add_jump_symbol(&mut self, symbol: &'a str, address: u16) {
        self.table.insert(symbol, address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_address() {
        let mut symbols = Symbols::new();
        assert_eq!(symbols.get_address("R0"), 0);
        assert_eq!(symbols.get_address("R8"), 8);
        assert_eq!(symbols.get_address("R15"), 15);
        assert_eq!(symbols.get_address("SCREEN"), 16384);
        assert_eq!(symbols.get_address("KBD"), 24576);
        assert_eq!(symbols.get_address("SP"), 0);
        assert_eq!(symbols.get_address("LCL"), 1);
        assert_eq!(symbols.get_address("ARG"), 2);
        assert_eq!(symbols.get_address("THIS"), 3);
        assert_eq!(symbols.get_address("THAT"), 4);
    }

    #[test]
    fn new_symbols() {
        let mut symbols = Symbols::new();
        assert_eq!(symbols.get_address("FOO"), 16);
        assert_eq!(symbols.get_address("BAR"), 17);
        assert_eq!(symbols.get_address("BAZ"), 18);
    }

    #[test]
    fn add_jump_symbol() {
        let mut symbols = Symbols::new();
        symbols.add_jump_symbol("FOO", 42);
        assert_eq!(symbols.get_address("FOO"), 42);
    }
}
