use crate::parser::{Command, Op, Segment};

pub struct CodeGen {
    current_filename: String,
    goto_index: usize,
    assembly_code: Vec<String>,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            current_filename: String::from("Global"),
            goto_index: 0,
            assembly_code: Vec::new(),
        }
    }

    fn get_current_filename(&self) -> &String {
        &self.current_filename
    }

    pub fn set_current_filename(&mut self, filename: &str) {
        self.current_filename = filename.to_string();
    }

    pub fn get_assembly_code(&self) -> &Vec<String> {
        &self.assembly_code
    }

    fn pop_stack_to_d(&mut self) {
        // SP--
        // D = *SP
        self.assembly_code.push(String::from("@SP"));
        self.assembly_code.push(String::from("AM=M-1"));
        self.assembly_code.push(String::from("D=M"));
    }

    fn push_d_to_stack(&mut self) {
        // *SP = D
        self.assembly_code.push(String::from("@SP"));
        self.assembly_code.push(String::from("A=M"));
        self.assembly_code.push(String::from("M=D"));

        // SP++
        self.assembly_code.push(String::from("@SP"));
        self.assembly_code.push(String::from("M=M+1"));
    }

    fn gen_arithmetic(&mut self, op: Op) {
        match op {
            Op::Add | Op::Subtract => {
                self.pop_stack_to_d();

                // @R13 = D
                self.assembly_code.push(String::from("@R13"));
                self.assembly_code.push(String::from("M=D"));

                self.pop_stack_to_d();

                // D = @R13 op D
                self.assembly_code.push(String::from("@R13"));
                if op == Op::Add {
                    self.assembly_code.push(String::from("D=D+M"));
                } else {
                    self.assembly_code.push(String::from("D=D-M"));
                }

                self.push_d_to_stack();
            }
            Op::Negate | Op::Not => {
                self.pop_stack_to_d();

                if op == Op::Negate {
                    self.assembly_code.push(String::from("D=-D"));
                } else {
                    self.assembly_code.push(String::from("D=!D"));
                }

                self.push_d_to_stack();
            }
            Op::Equal | Op::GreaterThan | Op::LessThan => {
                self.pop_stack_to_d();

                // @R13 = D
                self.assembly_code.push(String::from("@R13"));
                self.assembly_code.push(String::from("M=D"));

                self.pop_stack_to_d();

                // D=-1 IFF D COMP @R13
                // D=0 IFF D COMP @R13
                self.assembly_code.push(String::from("@R13"));
                self.assembly_code.push(String::from("D=D-M"));
                self.assembly_code
                    .push(format!("@TRUTHY{}", self.goto_index));

                match op {
                    Op::Equal => {
                        self.assembly_code.push(String::from("D;JEQ"));
                    }
                    Op::GreaterThan => {
                        self.assembly_code.push(String::from("D;JGT"));
                    }
                    _ => {
                        self.assembly_code.push(String::from("D;JLT"));
                    }
                }

                self.assembly_code.push(String::from("D=0"));
                self.assembly_code
                    .push(format!("@FALSY{}", self.goto_index));
                self.assembly_code.push(String::from("0;JMP"));

                self.assembly_code
                    .push(format!("(TRUTHY{})", self.goto_index));
                self.assembly_code.push(String::from("D=-1"));
                self.assembly_code
                    .push(format!("(FALSY{})", self.goto_index));

                self.push_d_to_stack();
                self.goto_index += 1;
            }
            Op::And | Op::Or => {
                self.pop_stack_to_d();

                // @R13 = D
                self.assembly_code.push(String::from("@R13"));
                self.assembly_code.push(String::from("M=D"));

                self.pop_stack_to_d();

                // D = D AND/OR @R13
                self.assembly_code.push(String::from("@R13"));
                if op == Op::And {
                    self.assembly_code.push(String::from("D=D&M"));
                } else {
                    self.assembly_code.push(String::from("D=D|M"));
                }

                self.push_d_to_stack();
            }
        }
    }

    fn assign_segment_index_to_d(&mut self, index: usize) {
        self.assembly_code.push(format!("@{}", index));
        self.assembly_code.push(String::from("A=D+A"));
        self.assembly_code.push(String::from("D=M"));
    }

    fn gen_push(&mut self, segment: Segment, index: usize) {
        // D = *segment[index]
        match segment {
            Segment::Argument => {
                self.assembly_code.push(String::from("@ARG"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_d(index);
            }
            Segment::Local => {
                self.assembly_code.push(String::from("@LCL"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_d(index);
            }
            Segment::Temp => {
                self.assembly_code.push(String::from("@5"));
                self.assembly_code.push(String::from("D=A"));
                self.assign_segment_index_to_d(index);
            }
            Segment::Pointer => {
                self.assembly_code.push(String::from("@3"));
                self.assembly_code.push(String::from("D=A"));
                self.assign_segment_index_to_d(index);
            }
            Segment::This => {
                self.assembly_code.push(String::from("@THIS"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_d(index);
            }
            Segment::That => {
                self.assembly_code.push(String::from("@THAT"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_d(index);
            }
            Segment::Static => {
                self.assembly_code
                    .push(format!("@{}.{}", self.current_filename, index));
                self.assembly_code.push(String::from("D=M"));
            }
            Segment::Constant => {
                self.assembly_code.push(format!("@{}", index));
                self.assembly_code.push(String::from("D=A"));
            }
        };

        self.push_d_to_stack();
    }

    fn assign_segment_index_to_temp(&mut self, index: usize) {
        self.assembly_code.push(format!("@{}", index));
        self.assembly_code.push(String::from("D=D+A"));
        // @R13 is a general purpose register
        self.assembly_code.push(String::from("@R13"));
        self.assembly_code.push(String::from("M=D"));
    }

    fn gen_pop(&mut self, segment: Segment, index: usize) {
        // @R13 = segment[index]
        match segment {
            Segment::Argument => {
                self.assembly_code.push(String::from("@ARG"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::Local => {
                self.assembly_code.push(String::from("@LCL"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::Temp => {
                self.assembly_code.push(String::from("@5"));
                self.assembly_code.push(String::from("D=A"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::Pointer => {
                self.assembly_code.push(String::from("@3"));
                self.assembly_code.push(String::from("D=A"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::This => {
                self.assembly_code.push(String::from("@THIS"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::That => {
                self.assembly_code.push(String::from("@THAT"));
                self.assembly_code.push(String::from("D=M"));
                self.assign_segment_index_to_temp(index);
            }
            Segment::Static => {
                self.assembly_code
                    .push(format!("@{}.{}", self.current_filename, index));
                self.assembly_code.push(String::from("D=A"));
                // @R13 is a general purpose register
                self.assembly_code.push(String::from("@R13"));
                self.assembly_code.push(String::from("M=D"));
            }
            Segment::Constant => panic!("Cannot pop to the constant segment."),
        };

        self.pop_stack_to_d();

        // *@R13 = D
        self.assembly_code.push(String::from("@R13"));
        self.assembly_code.push(String::from("A=M"));
        self.assembly_code.push(String::from("M=D"));
    }

    pub fn gen_label(&mut self, label: String) {
        self.assembly_code.push(format!("({})", label));
    }

    pub fn gen_goto(&mut self, label: String) {
        self.assembly_code.push(format!("@{}", label));
        self.assembly_code.push(String::from("0;JMP"));
    }

    pub fn gen_if_goto(&mut self, label: String) {
        self.pop_stack_to_d();
        self.assembly_code.push(format!("@{}", label));
        self.assembly_code.push(String::from("D;JNE"));
    }

    pub fn gen_command(&mut self, command: Command) {
        match command {
            Command::Arithmetic(op) => {
                self.gen_arithmetic(op);
            }

            Command::Push(segment, index) => {
                self.gen_push(segment, index);
            }

            Command::Pop(segment, index) => {
                self.gen_pop(segment, index);
            }

            Command::Label(label) => {
                self.gen_label(label);
            }

            Command::Goto(label) => {
                self.gen_goto(label);
            }

            Command::IfGoto(label) => {
                self.gen_if_goto(label);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_code_gen() {
        let code_gen = CodeGen::new();
        assert_eq!(code_gen.get_current_filename(), &String::from("Global"));
        assert_eq!(code_gen.get_assembly_code(), &Vec::<String>::new());
    }

    #[test]
    fn set_current_filename() {
        let mut code_gen = CodeGen::new();
        code_gen.set_current_filename("Foo");
        assert_eq!(code_gen.get_current_filename(), &String::from("Foo"));
    }

    #[test]
    fn gen_push() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Push(Segment::Argument, 3));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // D = *ARG[3]
                String::from("@ARG"),
                String::from("D=M"),
                String::from("@3"),
                String::from("A=D+A"),
                String::from("D=M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn gen_push_static() {
        let mut code_gen = CodeGen::new();
        code_gen.set_current_filename("Bar");

        code_gen.gen_command(Command::Push(Segment::Static, 3));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // D = *@Bar.3
                String::from("@Bar.3"),
                String::from("D=M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn gen_push_constant() {
        let mut code_gen = CodeGen::new();

        code_gen.gen_command(Command::Push(Segment::Constant, 3));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // D = 3
                String::from("@3"),
                String::from("D=A"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn gen_pop() {
        let mut code_gen = CodeGen::new();

        code_gen.gen_command(Command::Pop(Segment::Local, 2));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // @R13 = LCL[2]
                String::from("@LCL"),
                String::from("D=M"),
                String::from("@2"),
                String::from("D=D+A"),
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // *@R13 = D
                String::from("@R13"),
                String::from("A=M"),
                String::from("M=D")
            ]
        )
    }

    #[test]
    fn gen_pop_static() {
        let mut code_gen = CodeGen::new();
        code_gen.set_current_filename("Bar");

        code_gen.gen_command(Command::Pop(Segment::Static, 2));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // @R13 = @Bar.2
                String::from("@Bar.2"),
                String::from("D=A"),
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // *@R13 = D
                String::from("@R13"),
                String::from("A=M"),
                String::from("M=D")
            ]
        )
    }

    #[test]
    #[should_panic]
    fn gen_pop_constant() {
        let mut code_gen = CodeGen::new();

        code_gen.gen_command(Command::Pop(Segment::Constant, 3));
    }

    #[test]
    fn add() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Add));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D += @R13
                String::from("@R13"),
                String::from("D=D+M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn sub() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Subtract));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = @R13 - D
                String::from("@R13"),
                String::from("D=D-M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn neg() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Negate));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D =  -D
                String::from("D=-D"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn and() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::And));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = @R13 & D
                String::from("@R13"),
                String::from("D=D&M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn or() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Or));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = @R13 & D
                String::from("@R13"),
                String::from("D=D|M"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn not() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Not));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = !D
                String::from("D=!D"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn eq() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::Equal));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = -1 IFF D COMP @R13
                // D = 0 IFF D COMP @R13
                String::from("@R13"),
                String::from("D=D-M"),
                String::from("@TRUTHY0"),
                String::from("D;JEQ"),
                String::from("D=0"),
                String::from("@FALSY0"),
                String::from("0;JMP"),
                String::from("(TRUTHY0)"),
                String::from("D=-1"),
                String::from("(FALSY0)"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn gt() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::GreaterThan));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = -1 IFF D COMP @R13
                // D = 0 IFF D COMP @R13
                String::from("@R13"),
                String::from("D=D-M"),
                String::from("@TRUTHY0"),
                String::from("D;JGT"),
                String::from("D=0"),
                String::from("@FALSY0"),
                String::from("0;JMP"),
                String::from("(TRUTHY0)"),
                String::from("D=-1"),
                String::from("(FALSY0)"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn lt() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Arithmetic(Op::LessThan));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // @R13 = D
                String::from("@R13"),
                String::from("M=D"),
                // SP--
                // D = *SP
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                // D = -1 IFF D COMP @R13
                // D = 0 IFF D COMP @R13
                String::from("@R13"),
                String::from("D=D-M"),
                String::from("@TRUTHY0"),
                String::from("D;JLT"),
                String::from("D=0"),
                String::from("@FALSY0"),
                String::from("0;JMP"),
                String::from("(TRUTHY0)"),
                String::from("D=-1"),
                String::from("(FALSY0)"),
                // *SP = D
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                // SP++
                String::from("@SP"),
                String::from("M=M+1"),
            ]
        )
    }

    #[test]
    fn label() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Label(String::from("SOME_LABEL")));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![String::from("(SOME_LABEL)")]
        )
    }

    #[test]
    fn goto() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::Goto(String::from("SOME_LABEL")));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![String::from("@SOME_LABEL"), String::from("0;JMP")]
        )
    }

    #[test]
    fn if_goto() {
        let mut code_gen = CodeGen::new();
        code_gen.gen_command(Command::IfGoto(String::from("SOME_LABEL")));

        assert_eq!(
            code_gen.get_assembly_code(),
            &vec![
                // SP--
                String::from("@SP"),
                String::from("AM=M-1"),
                // D = *SP
                String::from("D=M"),
                // if D != 0 JUMP
                String::from("@SOME_LABEL"),
                String::from("D;JNE")
            ]
        )
    }
}
