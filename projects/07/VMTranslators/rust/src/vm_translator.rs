use crate::code_gen::CodeGen;
use crate::parser::Parser;
use std::path::Path;

pub struct Translator {
    code_gen: CodeGen,
}

impl Translator {
    pub fn new() -> Translator {
        Translator {
            code_gen: CodeGen::new(),
        }
    }

    pub fn translate(&mut self, filename: String, file_contents: String) {
        let file_stem = Path::new(&filename).file_stem().unwrap().to_str().unwrap();
        self.code_gen.set_current_filename(file_stem);
        let mut parser = Parser::new(file_contents);

        while parser.has_more_commands() {
            self.code_gen.gen_command(parser.parse_command());
            parser.advance();
        }
    }

    pub fn output(&self) -> String {
        self.code_gen.output().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let mut translator = Translator::new();

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
            "@256",
            "D=A",
            "@R0",
            "M=D",
            "@Sys.init",
            "0;JMP",
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
            "@foo.2",
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
            "(Sys.end)",
            "@Sys.end",
            "0;JMP",
        ]
        .join("\n");

        translator.translate(String::from("foo"), vm_program);

        assert_eq!(translator.output(), assembly_code);
    }
}
