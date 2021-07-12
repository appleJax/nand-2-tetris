use crate::code_gen::generate_command;
use crate::parser::{CommandType, Parser};
use crate::symbols::Symbols;

pub struct Assembler {}

impl Assembler {
    pub fn assemble(raw_program: String) -> String {
        let mut parser = Parser::new(&raw_program);
        let mut symbol_table = Symbols::new();

        // first pass to record jump symbols
        let mut line_index = 0;
        while parser.has_more_commands() {
            match parser.command_type() {
                CommandType::Jump => {
                    symbol_table.add_jump_symbol(parser.get_symbol(), line_index);
                }
                _ => {
                    line_index += 1;
                }
            }
            parser.advance();
        }

        parser.reset();

        // start assembly
        let mut assembly_code = String::new();
        while parser.has_more_commands() {
            match parser.command_type() {
                CommandType::Address => {
                    let symbol = parser.get_symbol();
                    let address = symbol
                        .parse::<u16>()
                        .unwrap_or_else(|_| symbol_table.get_address(symbol));

                    let binary_str = format!("{:0>16b}", address);
                    assembly_code.push_str(&binary_str);
                    assembly_code.push_str("\n");
                }
                CommandType::Computation => {
                    let command = parser.parse_command();
                    assembly_code.push_str(&generate_command(command));
                    assembly_code.push_str("\n");
                }
                _ => {}
            }

            parser.advance();
        }

        assembly_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble() {
        let program = "
           // Test Program
           
           @42
           D=M;JLT // will never jump
           
           @foo // not used
           @bar
           M=D
           
           (END)
           @END
           0;JMP
           
        "
        .to_string();

        let mut output = String::new();
        output.push_str("0000000000101010\n");
        output.push_str("1111110000010100\n");
        output.push_str("0000000000010000\n");
        output.push_str("0000000000010001\n");
        output.push_str("1110001100001000\n");
        output.push_str("0000000000000101\n");
        output.push_str("1110101010000111\n");

        assert_eq!(Assembler::assemble(program), output);
    }
}
