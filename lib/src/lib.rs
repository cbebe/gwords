// TODO: handle non-Hangul strings and characters
pub mod letter;

pub fn gword(phrase: &str) -> String {
    let mut s = String::new();
    for c in phrase.chars() {
        match letter::g_wordify(c) {
            Ok([a, b]) => {
                s.push(a);
                s.push(b);
            }
            Err(_) => s.push(c),
        }
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gword() {
        assert_eq!("아간녀겅하가세게요고", gword("안녕하세요"));
        assert_eq!("아간녀겅하가세게요고!", gword("안녕하세요!"));
        assert_eq!("가감사가하갑니기다가", gword("감사합니다"));
        assert_eq!("hello", gword("hello"));
    }
}
