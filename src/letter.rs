const FIRST_HANGUL_UNICODE: u32 = 0xac00; // 가
const LAST_HANGUL_UNICODE: u32 = 0xd7a3; // 힣

const NUM_CHO: u32 = 19;
const NUM_JOONG: u32 = 21;
const NUM_JONG: u32 = 28;

const NULL: char = '\0';

// 초성
const CHO: [char; NUM_CHO as usize] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

// 중성
const JOONG: [char; NUM_JOONG as usize] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];

fn get_ext(c: char) -> char {
    match c {
        'ㅑ' | 'ㅘ' => 'ㅏ',
        'ㅖ' | 'ㅞ' => 'ㅔ',
        'ㅒ' | 'ㅙ' => 'ㅐ',
        'ㅟ' | 'ㅢ' => 'ㅣ',
        'ㅕ' | 'ㅝ' => 'ㅓ',
        'ㅛ' => 'ㅗ',
        'ㅜ' => 'ㅜ',
        _ => c,
    }
}

// 종성
const JONG: [char; NUM_JONG as usize] = [
    NULL, 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
    'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

trait Get<T> {
    fn g(&self, idx: u32) -> T;
    fn index_of(&self, c: T) -> Option<usize>;
    fn in_arr(&self, c: T, idx: usize) -> Option<[T; 3]>;
}

impl Get<char> for [char] {
    fn g(&self, idx: u32) -> char {
        self[idx as usize]
    }
    fn index_of(&self, c: char) -> Option<usize> {
        self.iter().position(|&a| c == a)
    }

    fn in_arr(&self, c: char, idx: usize) -> Option<[char; 3]> {
        if self.index_of(c).is_some() {
            let mut arr = [NULL, NULL, NULL];
            arr[idx] = c;
            return Some(arr);
        }
        None
    }
}

fn decompose_from_code(c: char) -> [char; 3] {
    let mut code = c as u32 - FIRST_HANGUL_UNICODE;
    let jong = JONG.g(code % NUM_JONG);
    code /= NUM_JONG as u32;
    let joong = JOONG.g(code % NUM_JOONG);
    let cho = CHO.g(code / NUM_JOONG);

    [cho, joong, jong]
}

fn decompose(c: char) -> [char; 3] {
    if let Some(a) = CHO.in_arr(c, 0) {
        return a;
    }
    if let Some(a) = JOONG.in_arr(c, 1) {
        return a;
    }
    if let Some(a) = JONG.in_arr(c, 2) {
        return a;
    }

    if ((c as u32) < FIRST_HANGUL_UNICODE) || ((c as u32) >= LAST_HANGUL_UNICODE) {
        println!("oops: {:X}", c as u32);
        return [NULL, NULL, NULL];
    }

    decompose_from_code(c)
}

// Code = FIRST_HANGUL_UNICODE (초성 index * NUM_JOONG * NUM_JONG) + (중성 index * NUM_JONG) + 종성
fn compose_lvt(cho: char, joong: char, jong: char) -> Option<char> {
    let cho_idx = CHO.index_of(cho)?;
    let joong_idx = JOONG.index_of(joong)?;
    let jong_idx = JONG.index_of(jong)?;
    std::char::from_u32(
        FIRST_HANGUL_UNICODE
            + cho_idx as u32 * NUM_JOONG * NUM_JONG
            + joong_idx as u32 * NUM_JONG
            + jong_idx as u32,
    )
}

fn compose(cho: char, joong: char) -> Option<char> {
    compose_lvt(cho, joong, NULL)
}

pub fn g_wordify(c: char) -> Result<[char; 2], &'static str> {
    let [cho, joong, jong] = decompose(c);
    let first = match compose(cho, joong) {
        Some(a) => a,
        None => return Err("Not Hangul"),
    };
    let second = match compose_lvt('ㄱ', get_ext(joong), jong) {
        Some(a) => a,
        None => return Err("Not Hangul"),
    };
    Ok([first, second])
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decompose() {
        assert_eq!(['ㄱ', 'ㅗ', 'ㅅ'], decompose('곳'));
        assert_eq!(['ㄱ', 'ㅏ', NULL], decompose('가'));
        assert_eq!(['ㄱ', NULL, NULL], decompose('ㄱ'));
        assert_eq!([NULL, 'ㅙ', NULL], decompose('ㅙ'));
        assert_eq!([NULL, NULL, 'ㅄ'], decompose('ㅄ'));
        assert_eq!([NULL, NULL, NULL], decompose('a'));
    }

    #[test]
    fn test_compose() {
        assert_eq!(Some('감'), compose_lvt('ㄱ', 'ㅏ', 'ㅁ'));
        assert_eq!(Some('부'), compose('ㅂ', 'ㅜ'));
    }

    #[test]
    fn test_g_wordify() {
        assert_eq!(Ok(['가', '가']), g_wordify('가'));
        assert_eq!(Ok(['뿌', '굼']), g_wordify('뿜'));
        assert_eq!(Ok(['왜', '개']), g_wordify('왜'));
        assert_eq!(Ok(['퀴', '긱']), g_wordify('퀵'));
        assert_eq!(Err("Not Hangul"), g_wordify('a'));
    }
}
