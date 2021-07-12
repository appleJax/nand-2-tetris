use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub dest: String,
    pub comp: String,
    pub jump: String,
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Address,
    Jump,
    Computation,
}

pub struct Parser {
    current_line: usize,
    program: Vec<String>,
}

impl Parser {
    pub fn new(raw_program: &str) -> Parser {
        Parser {
            current_line: 0,
            program: Parser::normalize_program(raw_program),
        }
    }

    fn normalize_program(raw_program: &str) -> Vec<String> {
        raw_program
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
        let trimmed = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }

    fn get_program(&self) -> &Vec<String> {
        &self.program
    }

    fn get_current_line(&self) -> usize {
        self.current_line
    }

    pub fn has_more_commands(&self) -> bool {
        self.current_line < self.program.len()
    }

    pub fn reset(&mut self) {
        self.current_line = 0;
    }

    pub fn advance(&mut self) {
        if self.has_more_commands() {
            self.current_line += 1;
        }
    }

    pub fn command_type(&self) -> CommandType {
        let command = self.program.get(self.current_line).unwrap();
        let first_char = command.chars().nth(0).unwrap();
        match first_char {
            '@' => CommandType::Address,
            '(' => CommandType::Jump,
            _ => CommandType::Computation,
        }
    }

    pub fn get_symbol(&self) -> String {
        let line = &self.program[self.current_line];
        match self.command_type() {
            CommandType::Address => line[1..].to_string(),
            CommandType::Jump => line[1..line.len() - 1].to_string(),
            _ => line.clone(),
        }
    }

    pub fn parse_command(&self) -> Command {
        let line = &self.program[self.current_line];

        let mut dest = String::new();
        let comp;
        let mut jump = String::new();

        let dest_end_index = line.find("=");
        let jump_start_index = line.find(";");

        match (dest_end_index, jump_start_index) {
            (Some(i), Some(j)) => {
                dest = line[..i].to_string();
                comp = line[i + 1..j].to_string();
                jump = line[j + 1..].to_string();
            }

            (Some(i), None) => {
                dest = line[..i].to_string();
                comp = line[i + 1..].to_string();
            }

            (None, Some(j)) => {
                comp = line[..j].to_string();
                jump = line[j + 1..].to_string();
            }

            (None, None) => {
                comp = line[..].to_string();
            }
        }

        Command { dest, comp, jump }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_parser() {
        let program = "
        // some header comment
        
        @R0
        D = M

        0;JMP // inline comment

        ";

        assert_eq!(
            Parser::new(program).get_program().join("\n"),
            "@R0\nD=M\n0;JMP"
        );
    }

    #[test]
    fn command_type() {
        let a_command = "@R0";
        let l_command = "(LOOP)";
        let c_command = "D=D+M;JGE";

        assert_eq!(Parser::new(a_command).command_type(), CommandType::Address);

        assert_eq!(Parser::new(l_command).command_type(), CommandType::Jump);

        assert_eq!(
            Parser::new(c_command).command_type(),
            CommandType::Computation
        );
    }

    #[test]
    fn get_symbol() {
        let a_command = "@R0";
        let l_command = "(LOOP)";

        assert_eq!(Parser::new(a_command).get_symbol(), "R0");
        assert_eq!(Parser::new(l_command).get_symbol(), "LOOP");
    }

    #[test]
    fn line_counter() {
        let program = "
            D=D-M;JEQ
            M=!A
        ";
        let mut parser = Parser::new(program);

        assert_eq!(parser.get_current_line(), 0);
        assert!(parser.has_more_commands());

        parser.advance();
        assert_eq!(parser.get_current_line(), 1);
        assert!(parser.has_more_commands());

        parser.advance();
        assert_eq!(parser.get_current_line(), 2);
        assert_ne!(parser.has_more_commands(), true);

        parser.advance();
        assert_eq!(parser.get_current_line(), 2);
        assert_ne!(parser.has_more_commands(), true);

        parser.reset();
        assert_eq!(parser.get_current_line(), 0);
        assert!(parser.has_more_commands());
    }

    #[test]
    fn parse_command() {
        let program = "
            D=D-M;JEQ
            M=!A
            0;JMP
        ";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse_command(),
            Command {
                dest: String::from("D"),
                comp: String::from("D-M"),
                jump: String::from("JEQ")
            }
        );

        parser.advance();
        assert_eq!(
            parser.parse_command(),
            Command {
                dest: String::from("M"),
                comp: String::from("!A"),
                jump: String::new()
            }
        );

        parser.advance();
        assert_eq!(
            parser.parse_command(),
            Command {
                dest: String::new(),
                comp: String::from("0"),
                jump: String::from("JMP")
            }
        );
    }
}
