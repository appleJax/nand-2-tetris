pub struct Command<'a> {
    dest: &'a str,
    comp: &'a str,
    jump: &'a str,
}

pub fn generate_command(command: Command) -> String {
    let Command { dest, comp, jump } = command;
    format!(
        "111{}{}{}",
        generate_comp(comp),
        generate_dest(dest),
        generate_jump(jump)
    )
}

fn generate_dest(dest: &str) -> String {
    match dest {
        "M" => String::from("001"),
        "D" => String::from("010"),
        "MD" => String::from("011"),
        "A" => String::from("100"),
        "AM" => String::from("101"),
        "AD" => String::from("110"),
        "AMD" => String::from("111"),
        _ => String::from("000"),
    }
}

fn generate_comp(comp: &str) -> String {
    match comp {
        "0" => String::from("0101010"),
        "1" => String::from("0111111"),
        "-1" => String::from("0111010"),
        "D" => String::from("0001100"),
        "A" => String::from("0110000"),
        "M" => String::from("1110000"),
        "!D" => String::from("0001101"),
        "!A" => String::from("0110001"),
        "!M" => String::from("1110001"),
        "-D" => String::from("0001111"),
        "-A" => String::from("0110011"),
        "-M" => String::from("1110011"),
        "D+1" => String::from("0011111"),
        "A+1" => String::from("0110111"),
        "M+1" => String::from("1110111"),
        "D-1" => String::from("0001110"),
        "A-1" => String::from("0110010"),
        "M-1" => String::from("1110010"),
        "D+A" => String::from("0000010"),
        "D+M" => String::from("1000010"),
        "D-A" => String::from("0010011"),
        "D-M" => String::from("1010011"),
        "A-D" => String::from("0000111"),
        "M-D" => String::from("1000111"),
        "D&A" => String::from("0000000"),
        "D&M" => String::from("1000000"),
        "D|A" => String::from("0010101"),
        "D|M" => String::from("1010101"),
        _ => panic!("Invalid computation: {}", comp),
    }
}

fn generate_jump(jump: &str) -> String {
    match jump {
        "JGT" => String::from("001"),
        "JEQ" => String::from("010"),
        "JGE" => String::from("011"),
        "JLT" => String::from("100"),
        "JNE" => String::from("101"),
        "JLE" => String::from("110"),
        "JMP" => String::from("111"),
        _ => String::from("000"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_command() {
        let command = Command {
            dest: "MD",
            comp: "D+M",
            jump: "JGE",
        };
        let binary_command = generate_command(command);
        assert_eq!(binary_command, String::from("1111000010011011"));
    }

    #[test]
    fn no_dest() {
        let command = Command {
            dest: "",
            comp: "D+M",
            jump: "JGE",
        };
        let binary_command = generate_command(command);
        assert_eq!(binary_command, String::from("1111000010000011"));
    }

    #[test]
    fn no_jump() {
        let command = Command {
            dest: "MD",
            comp: "D+M",
            jump: "",
        };
        let binary_command = generate_command(command);
        assert_eq!(binary_command, String::from("1111000010011000"));
    }

    #[test]
    #[should_panic]
    fn no_comp() {
        let command = Command {
            dest: "MD",
            comp: "",
            jump: "JGE",
        };
        generate_command(command);
    }
}
