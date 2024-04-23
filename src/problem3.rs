use crate::common::*;

enum CommaKind {
    Canada,
    Harvard,
}

impl CommaKind {
    fn join(&self, words: Vec<String>, joinword: String) -> String {
        if words.len() == 1 {
            return words[0].clone();
        }

        if words.len() == 2 {
            return format!("{} {} {}", words[0], joinword, words[1]);
        }

        let commaed = words[0..words.len() - 1].join(", ");
        let spec = match self {
            Self::Canada => format!(" {} {}", joinword, words[words.len() - 1]),
            Self::Harvard => format!(", {} {}", joinword, words[words.len() - 1]),
        };

        format!("{commaed}{spec}")
    }

    fn parse(s: String) -> Self {
        match &s as &str {
            "HARVARD" => Self::Harvard,
            "CANADA" => Self::Canada,
            _ => panic!(),
        }
    }
}

pub fn main() {
    let mut words = vec![];

    loop {
        let inp = read_line::<String>();

        words.push(inp.clone());

        match &inp as &str {
            "HARVARD" => break,
            "CANADA" => break,
            _ => {}
        }
    }

    let style = CommaKind::parse(words.pop().unwrap());
    let joinword = words.pop().unwrap();

    println!("{}", style.join(words, joinword));
}
