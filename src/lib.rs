// TODO: handle non-Hangul strings and characters
pub mod letter;

pub fn gword(str: &str) -> String {
    str.chars().fold(String::new(), |mut s, c| {
        let [a, b] = letter::g_wordify(c);
        s.push(a);
        s.push(b);
        s
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gword() {
        assert_eq!("아간녀겅하가세게요고", gword("안녕하세요"));
        assert_eq!("가감사가하갑니기다가", gword("감사합니다"));
    }
}
