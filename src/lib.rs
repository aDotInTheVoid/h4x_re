const START: u8 = b'^';
const DOT: u8 = b'.';
const END: u8 = b'$';

#[derive(PartialEq, Debug, Clone)]
pub struct Regex<'a> {
    binds: Binds,
    pattern: Pattern<'a>,
}

#[derive(PartialEq, Debug, Clone)]
enum Binds {
    Front,
    Back,
    Both,
    Neither,
}

#[derive(PartialEq, Debug, Clone)]
enum Pattern<'a> {
    NoDots(&'a str),
    Dots(&'a str),
}

impl Pattern<'_> {
    fn len(&self) -> usize {
        self.str().len()
    }

    fn str(&self) -> &str {
        match self {
            Self::Dots(x) => x,
            Self::NoDots(x) => x,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        self.str().as_bytes()
    }
}

impl<'a> Regex<'a> {
    pub fn new(input: &'a str) -> Self {
        // Check for ^ and $ in regex
        let has_start = input.as_bytes()[0] == START;
        let has_end = input.as_bytes().last().map(|&x| x == END).unwrap_or(false);

        let start_idx = if has_start { 1 } else { 0 };
        let end_idx = if has_end {
            input.len() - 1
        } else {
            input.len()
        };

        let binds = match (has_start, has_end) {
            (true, true) => Binds::Both,
            (true, false) => Binds::Front,
            (false, true) => Binds::Back,
            (false, false) => Binds::Neither,
        };

        let pattern_range = &input[start_idx..end_idx];
        let pattern = if input.as_bytes().contains(&DOT) {
            Pattern::Dots(pattern_range)
        } else {
            Pattern::NoDots(pattern_range)
        };

        Self { binds, pattern }
    }

    pub fn is_match(&self, text: &str) -> bool {
        let (start, end) = match self.binds {
            Binds::Front => (0, self.pattern.len()),
            Binds::Back => (
                match text.len().checked_sub(self.pattern.len()) {
                    Some(x) => x,
                    None => return false,
                },
                text.len(),
            ),
            Binds::Both => {
                if text.len() == self.pattern.len() {
                    return self.match_knows_pos(text);
                } else {
                    return false;
                }
            }
            Binds::Neither => return self.match_unknown_pos(text),
        };
        text.get(start..end)
            .map(|x| self.match_knows_pos(x))
            .unwrap_or(false)
    }

    fn match_knows_pos(&self, text: &str) -> bool {
        match self.pattern {
            Pattern::NoDots(x) => x == text,
            Pattern::Dots(_) => self.match_dots_pos(text),
        }
    }

    fn match_unknown_pos(&self, text: &str) -> bool {
        match self.pattern {
            Pattern::NoDots(x) => text.contains(x),
            Pattern::Dots(_) => self.match_dots_pos_unknown(text),
        }
    }

    fn match_dots_pos(&self, text: &str) -> bool {
        if self.pattern.len() != text.len() {
            return false;
        };
        for (pat, txt) in self.pattern.as_bytes().iter().zip(text.as_bytes()) {
            if *pat == DOT {
                continue;
            } else if pat != txt {
                return false;
            }
        }
        true
    }

    fn match_dots_pos_unknown(&self, text: &str) -> bool {
        dbg!();
        if text.len() < self.pattern.len() {
            return false;
        }
        
        for i in 0..=text.len()-self.pattern.len() {
            if self.match_dots_pos(dbg!(&text[i..i+self.pattern.len()])) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let ans_1 = Regex {
            binds: Binds::Both,
            pattern: Pattern::NoDots("win"),
        };
        let ans_2 = Regex {
            binds: Binds::Front,
            pattern: Pattern::NoDots("win"),
        };
        let ans_3 = Regex {
            binds: Binds::Front,
            pattern: Pattern::Dots("wi."),
        };
        let ans_4 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::Dots("wi."),
        };
        let ans_5 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::NoDots("wi"),
        };
        let ans_6 = Regex {
            binds: Binds::Front,
            pattern: Pattern::NoDots("wi"),
        };
        let ans_7 = Regex {
            binds: Binds::Back,
            pattern: Pattern::NoDots("win"),
        };
        let ans_8 = Regex {
            binds: Binds::Neither,
            pattern: Pattern::NoDots("win"),
        };
        let ans_9 = Regex {
            binds: Binds::Back,
            pattern: Pattern::Dots("wi."),
        };

        let test_1 = "^win$";
        let test_2 = "^win";
        let test_3 = "^wi.";
        let test_4 = "wi.";
        let test_5 = "wi";
        let test_6 = "^wi";
        let test_7 = "win$";
        let test_8 = "win";
        let test_9 = "wi.$";
        assert_eq!(Regex::new(test_1), ans_1);
        assert_eq!(Regex::new(test_2), ans_2);
        assert_eq!(Regex::new(test_3), ans_3);
        assert_eq!(Regex::new(test_4), ans_4);
        assert_eq!(Regex::new(test_5), ans_5);
        assert_eq!(Regex::new(test_6), ans_6);
        assert_eq!(Regex::new(test_7), ans_7);
        assert_eq!(Regex::new(test_8), ans_8);
        assert_eq!(Regex::new(test_9), ans_9);
    }

    macro_rules! reg_text {
        ($regex:expr, [$($accpet:expr),*],  [$($regect:expr),*]) => {
            let re = Regex::new($regex);
            $(
                assert!(re.is_match($accpet));
            )*
            $(
                assert!(!re.is_match($regect));
            )*
        };
    }

    #[test]
    fn no_dots() {
        reg_text!("^win$", ["win"], ["", "winn", "wwin", "wi", "in", "banana"]);
        reg_text!(
            "^win",
            ["win", "win ", "windows XP"],
            [" win", "xw", "wi", "in", ""]
        );
        reg_text!(
            "wi",
            ["wi", "win", "pinwion", "Mario Kart wii"],
            ["w i", "we", "w"]
        );
        reg_text!(
            "win$",
            ["win", "xd win", "how to win"],
            ["banh", "win95", "n"]
        );
    }

    #[test]
    fn dots_known_pos() {
        reg_text!("^wi.", ["win", "wix", "windows 2000"], ["wi", "won"]);
        reg_text!("^w.n$", ["win", "wan", "wxn"], ["winn", " win ", "xin"]);
        reg_text!("..n$", ["win", "..n", "pin", "impl pin", "xdin"], ["in"]);
    }

    #[test]
    fn dots_unknown_pos() {
        reg_text!("w.n", [" win", "wxnxxx", "win", "winwinwin", "dsfawxndf"], ["wnwn", "dsasdf", "asdf", ""]);
        reg_text!("..x..", ["  x  ", "xxxxx"], ["sfsdfdx", "vxdfs", "asdfdsxd"]);
    }
}
