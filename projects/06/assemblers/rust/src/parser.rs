use regex::Regex;

pub struct Command<'a> {
    pub dest: &'a str,
    pub comp: &'a str,
    pub jump: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Address,
    Loop,
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

    fn strip_comments(line: String) -> String {
        let re = Regex::new("//.*").unwrap();
        re.replace(line, "")
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

    pub fn get_program(&self) -> &Vec<String> {
        &self.program
    }

    pub fn has_more_commands(&self) -> bool {
        self.current_line < self.program.len()
    }

    fn get_current_line(&self) -> usize {
        self.current_line
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
            '(' => CommandType::Loop,
            _ => CommandType::Computation,
        }
    }

    pub fn get_symbol(&self) -> String {
        let line = &self.program[self.current_line];
        match self.command_type() {
            CommandType::Address => line[1..].to_string(),
            CommandType::Loop => line[1..line.len() - 1].to_string(),
            _ => line.clone(),
        }
    }

    pub fn parse_command() -> Command {
        let dest = String::new();
        let jump: String::new();
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

        assert_eq!(Parser::new(l_command).command_type(), CommandType::Loop);

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
}
