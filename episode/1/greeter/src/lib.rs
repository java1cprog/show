use std::fmt;
use std::default;

#[derive(Debug)]
#[allow(dead_code)]
enum Language {
    English,
    German,
    Russian,
}
#[derive(Debug)]
struct Greeter {
    language: Language,
}

impl fmt::Display for Greeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let greeting = match self.language {
            Language::English => "Hello",
            Language::German => "Hallo",
            Language::Russian => "Привет",
        };
        write!(f, "{} Rust!", greeting)
    }
}

impl default::Default for Greeter {
    fn default()->Greeter{
        Greeter{
            language:Language::English,
        }
    }
}

#[allow(dead_code)]
impl Greeter {
    fn new(language: Language) -> Greeter {
        Greeter { language }
    }

    pub fn with_language(mut self, language: Language) -> Greeter {
        self.language = language;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let greeter:Greeter = Default::default();
        assert_eq!(format!("{}",greeter), "Hello Rust!");

    }

    #[test]
    fn it_works2() {
        let greeter = Greeter::new(Language::German);
        assert_eq!(format!("{}",greeter), "Hallo Rust!");
    }
    #[test]
    fn it_works3() {
        let mut  greeter = Greeter::new(Language::German);
        greeter = greeter.with_language(Language::Russian);
        assert_eq!(format!("{}",greeter), "Привет Rust!");
    }

}
