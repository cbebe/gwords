// TODO: handle non-Hangul strings and characters
pub mod letter;

pub fn gword(phrase: &str) -> Result<String, &'static str> {
    phrase.chars().try_fold(String::new(), |mut s, c| {
        letter::g_wordify(c).map(|[a, b]| {
            s.push(a);
            s.push(b);
            s
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gword() {
        assert_eq!(
            Ok(String::from("아간녀겅하가세게요고")),
            gword("안녕하세요")
        );
        assert_eq!(
            Ok(String::from("가감사가하갑니기다가")),
            gword("감사합니다")
        );
        assert_eq!(Err("Not Hangul"), gword("hello"));
    }
}
