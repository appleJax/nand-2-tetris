use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Segment {
    Local,
    Argument,
    Static,
    Constant,
    Temp,
    Pointer,
    This,
    That,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Negate,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Arithmetic(Op),
    Push(Segment, usize),
    Pop(Segment, usize),
    Label(String),
    Goto(String),
    IfGoto(String),
    Function,
    Return,
    Call,
}

pub struct Parser {
    current_line: usize,
    program: Vec<String>,
}

impl Parser {
    pub fn new(file_contents: String) -> Parser {
        Parser {
            current_line: 0,
            program: Parser::normalize_program(file_contents),
        }
    }

    fn normalize_program(file_contents: String) -> Vec<String> {
        file_contents
            .lines()
            .filter_map(|line| {
                let line = Parser::strip_comments(line);
                Parser::strip_whitespace(line)
            })
            .collect()
    }

    fn strip_comments(line: &str) -> String {
        let re = Regex::new("//.*").unwrap();
        re.replace(line, "").to_string()
    }

    fn strip_whitespace(line: String) -> Option<String> {
        let trim_edges_re = Regex::new("(^ +)|( +$)").unwrap();
        let line = trim_edges_re.replace_all(&line, "").to_string();

        let dedup_spaces_re = Regex::new(" +").unwrap();
        let line = dedup_spaces_re.replace_all(&line, " ").to_string();

        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }

    fn get_program(&self) -> &Vec<String> {
        &self.program
    }

    pub fn has_more_commands(&self) -> bool {
        self.current_line < self.program.len()
    }

    pub fn advance(&mut self) {
        if self.current_line < self.program.len() {
            self.current_line += 1;
        }
    }

    fn get_op(&self) -> Op {
        let command = &self.program[self.current_line];

        match command.as_str() {
            "add" => Op::Add,
            "sub" => Op::Subtract,
            "neg" => Op::Negate,
            "eq" => Op::Equal,
            "gt" => Op::GreaterThan,
            "lt" => Op::LessThan,
            "and" => Op::And,
            "or" => Op::Or,
            "not" => Op::Not,
            _ => panic!("Invalid arithmetic operation: {}", command),
        }
    }

    fn get_segment(&self) -> Segment {
        let command = &self.program[self.current_line];
        let mut parts = command.split_whitespace();

        parts.next();
        let segment = String::from(parts.next().unwrap());

        match segment.as_str() {
            "local" => Segment::Local,
            "argument" => Segment::Argument,
            "static" => Segment::Static,
            "constant" => Segment::Constant,
            "temp" => Segment::Temp,
            "pointer" => Segment::Pointer,
            "this" => Segment::This,
            "that" => Segment::That,
            _ => panic!("Invalid segment: {}", segment),
        }
    }

    fn get_index(&self) -> usize {
        let command = &self.program[self.current_line];
        let mut parts = command.split_whitespace();

        parts.next();
        parts.next();
        parts.next().unwrap().parse::<usize>().unwrap()
    }

    fn get_label(&self) -> String {
        let command = &self.program[self.current_line];
        let mut parts = command.split_whitespace();

        parts.next();
        String::from(parts.next().unwrap())
    }

    pub fn parse_command(&self) -> Command {
        let command = &self.program[self.current_line];

        match &command[..2] {
            "pu" => Command::Push(self.get_segment(), self.get_index()),
            "po" => Command::Pop(self.get_segment(), self.get_index()),
            "la" => Command::Label(self.get_label()),
            "go" => Command::Goto(self.get_label()),
            "if" => Command::IfGoto(self.get_label()),
            _ => Command::Arithmetic(self.get_op()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parser() {
        let program = "
        // This is a test program

        push argument 0 // x
        push   local 0 // y
        add

        // local 1 =   x  +  y
        pop local     1

        "
        .to_string();
        let parser = Parser::new(program);

        let normalized_program = vec![
            String::from("push argument 0"),
            String::from("push local 0"),
            String::from("add"),
            String::from("pop local 1"),
        ];
        assert_eq!(parser.get_program(), &normalized_program);
    }

    #[test]
    fn line_counter() {
        let program = "
        line 1
        line 2
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert!(parser.has_more_commands());

        parser.advance();
        assert!(parser.has_more_commands());

        parser.advance();
        assert_eq!(parser.has_more_commands(), false);
    }

    #[test]
    fn get_op() {
        let program = "
        add
        eq
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert_eq!(parser.get_op(), Op::Add);

        parser.advance();
        assert_eq!(parser.get_op(), Op::Equal);
    }

    #[test]
    #[should_panic]
    fn invalid_op() {
        let program = "
        mult
        "
        .to_string();

        let mut parser = Parser::new(program);
        parser.get_op();
    }

    #[test]
    fn get_segment() {
        let program = "
        push argument 1
        pop local 0
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert_eq!(parser.get_segment(), Segment::Argument);

        parser.advance();
        assert_eq!(parser.get_segment(), Segment::Local);
    }

    #[test]
    #[should_panic]
    fn invalid_segment() {
        let program = "
        pop foo 1
        "
        .to_string();

        let mut parser = Parser::new(program);
        parser.get_segment();
    }

    #[test]
    fn get_index() {
        let program = "
        push argument 3
        pop local 10
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert_eq!(parser.get_index(), 3);

        parser.advance();
        assert_eq!(parser.get_index(), 10);
    }

    #[test]
    fn get_label() {
        let program = "
        label SOME_LABEL
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert_eq!(parser.get_label(), String::from("SOME_LABEL"));
    }

    #[test]
    fn parse_command() {
        let program = "
        push argument 3
        pop local 10
        add
        label SOME_LABEL
        goto GOTO_LABEL
        if-goto IF_GOTO_LABEL
        "
        .to_string();

        let mut parser = Parser::new(program);

        assert_eq!(parser.parse_command(), Command::Push(Segment::Argument, 3));

        parser.advance();
        assert_eq!(parser.parse_command(), Command::Pop(Segment::Local, 10));

        parser.advance();
        assert_eq!(parser.parse_command(), Command::Arithmetic(Op::Add));

        parser.advance();
        assert_eq!(
            parser.parse_command(),
            Command::Label(String::from("SOME_LABEL"))
        );

        parser.advance();
        assert_eq!(
            parser.parse_command(),
            Command::Goto(String::from("GOTO_LABEL"))
        );

        parser.advance();
        assert_eq!(
            parser.parse_command(),
            Command::IfGoto(String::from("IF_GOTO_LABEL"))
        );
    }
}
