use crate::code_gen::CodeGen;
use crate::parser::Parser;

pub struct Translator {}

impl Translator {
    pub fn translate(raw_program: String) -> String {
        let mut parser = Parser::new(raw_program);
        let mut code_gen = CodeGen::new();

        while parser.has_more_commands() {
            code_gen.gen_command(parser.parse_command());
            parser.advance();
        }

        code_gen.get_assembly_code().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let vm_program = "
        // Test VM Program

        push argument 3
        push constant 5
        add
        pop local 0 // local[0] = arg[3] + 5

        push static 2
        push this 1
        gt

        not

        "
        .to_string();

        let assembly_code = vec![
            "@ARG",
            "D=M",
            "@3",
            "A=D+A",
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@5",
            "D=A",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@SP",
            "AM=M-1",
            "D=M",
            "@R13",
            "M=D",
            "@SP",
            "AM=M-1",
            "D=M",
            "@R13",
            "D=D+M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@LCL",
            "D=M",
            "@0",
            "D=D+A",
            "@R13",
            "M=D",
            "@SP",
            "AM=M-1",
            "D=M",
            "@R13",
            "A=M",
            "M=D",
            "@Global.2",
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@THIS",
            "D=M",
            "@1",
            "A=D+A",
            "D=M",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@SP",
            "AM=M-1",
            "D=M",
            "@R13",
            "M=D",
            "@SP",
            "AM=M-1",
            "D=M",
            "@R13",
            "D=D-M",
            "@TRUTHY0",
            "D;JGT",
            "D=0",
            "@FALSY0",
            "0;JMP",
            "(TRUTHY0)",
            "D=-1",
            "(FALSY0)",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
            "@SP",
            "AM=M-1",
            "D=M",
            "D=!D",
            "@SP",
            "A=M",
            "M=D",
            "@SP",
            "M=M+1",
        ]
        .join("\n");

        assert_eq!(Translator::translate(vm_program), assembly_code);
    }
}
