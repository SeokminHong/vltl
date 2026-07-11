use unicode_normalization::UnicodeNormalization;

const INITIAL_KEYS: [&str; 19] = [
    "r", "R", "s", "e", "E", "f", "a", "q", "Q", "t", "T", "d", "w", "W", "c", "z", "x", "v", "g",
];

const MEDIAL_KEYS: [&str; 21] = [
    "k", "o", "i", "O", "j", "p", "u", "P", "h", "hk", "ho", "hl", "y", "n", "nj", "np", "nl", "b",
    "m", "ml", "l",
];

const FINAL_KEYS: [&str; 28] = [
    "", "r", "R", "rt", "s", "sw", "sg", "e", "f", "fr", "fa", "fq", "ft", "fx", "fv", "fg", "a",
    "q", "qt", "t", "T", "d", "w", "c", "z", "x", "v", "g",
];

fn compatibility_jamo_keys(character: char) -> Option<&'static str> {
    match character {
        'ㄱ' => Some("r"),
        'ㄲ' => Some("R"),
        'ㄳ' => Some("rt"),
        'ㄴ' => Some("s"),
        'ㄵ' => Some("sw"),
        'ㄶ' => Some("sg"),
        'ㄷ' => Some("e"),
        'ㄸ' => Some("E"),
        'ㄹ' => Some("f"),
        'ㄺ' => Some("fr"),
        'ㄻ' => Some("fa"),
        'ㄼ' => Some("fq"),
        'ㄽ' => Some("ft"),
        'ㄾ' => Some("fx"),
        'ㄿ' => Some("fv"),
        'ㅀ' => Some("fg"),
        'ㅁ' => Some("a"),
        'ㅂ' => Some("q"),
        'ㅃ' => Some("Q"),
        'ㅄ' => Some("qt"),
        'ㅅ' => Some("t"),
        'ㅆ' => Some("T"),
        'ㅇ' => Some("d"),
        'ㅈ' => Some("w"),
        'ㅉ' => Some("W"),
        'ㅊ' => Some("c"),
        'ㅋ' => Some("z"),
        'ㅌ' => Some("x"),
        'ㅍ' => Some("v"),
        'ㅎ' => Some("g"),
        'ㅏ' => Some("k"),
        'ㅐ' => Some("o"),
        'ㅑ' => Some("i"),
        'ㅒ' => Some("O"),
        'ㅓ' => Some("j"),
        'ㅔ' => Some("p"),
        'ㅕ' => Some("u"),
        'ㅖ' => Some("P"),
        'ㅗ' => Some("h"),
        'ㅘ' => Some("hk"),
        'ㅙ' => Some("ho"),
        'ㅚ' => Some("hl"),
        'ㅛ' => Some("y"),
        'ㅜ' => Some("n"),
        'ㅝ' => Some("nj"),
        'ㅞ' => Some("np"),
        'ㅟ' => Some("nl"),
        'ㅠ' => Some("b"),
        'ㅡ' => Some("m"),
        'ㅢ' => Some("ml"),
        'ㅣ' => Some("l"),
        _ => None,
    }
}

fn canonical_jamo_keys(character: char) -> Option<&'static str> {
    let code = character as u32;

    match code {
        0x1100..=0x1112 => INITIAL_KEYS.get((code - 0x1100) as usize).copied(),
        0x1161..=0x1175 => MEDIAL_KEYS.get((code - 0x1161) as usize).copied(),
        0x11A8..=0x11C2 => FINAL_KEYS.get((code - 0x11A7) as usize).copied(),
        _ => None,
    }
}

fn append_syllable_keys(output: &mut String, character: char) -> bool {
    let code = character as u32;
    if !(0xAC00..=0xD7A3).contains(&code) {
        return false;
    }

    let syllable_index = code - 0xAC00;
    let initial_index = syllable_index / (21 * 28);
    let medial_index = (syllable_index % (21 * 28)) / 28;
    let final_index = syllable_index % 28;

    output.push_str(INITIAL_KEYS[initial_index as usize]);
    output.push_str(MEDIAL_KEYS[medial_index as usize]);
    output.push_str(FINAL_KEYS[final_index as usize]);
    true
}

pub fn contains_korean(input: &str) -> bool {
    input.chars().any(|character| {
        let code = character as u32;
        (0xAC00..=0xD7A3).contains(&code)
            || (0x3131..=0x318E).contains(&code)
            || (0x1100..=0x11FF).contains(&code)
            || (0xA960..=0xA97F).contains(&code)
            || (0xD7B0..=0xD7FF).contains(&code)
    })
}

/// 두벌식 한글 입력을 같은 키 위치의 영문 QWERTY 문자열로 변환합니다.
pub fn convert_korean_to_english(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for character in input.nfc() {
        if append_syllable_keys(&mut output, character) {
            continue;
        }

        if let Some(keys) =
            compatibility_jamo_keys(character).or_else(|| canonical_jamo_keys(character))
        {
            output.push_str(keys);
        } else {
            output.push(character);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{contains_korean, convert_korean_to_english};

    #[test]
    fn converts_simple_syllables() {
        assert_eq!(convert_korean_to_english("피"), "vl");
        assert_eq!(convert_korean_to_english("며"), "au");
        assert_eq!(convert_korean_to_english("내"), "so");
    }

    #[test]
    fn converts_incomplete_input() {
        assert_eq!(convert_korean_to_english("ㅍㅣ"), "vl");
        assert_eq!(convert_korean_to_english("ㅔㅞㅡ"), "pnpm");
        assert_eq!(convert_korean_to_english("ㅛㅁ구"), "yarn");
        assert_eq!(convert_korean_to_english("ㅎㄱ데"), "grep");
    }

    #[test]
    fn converts_compound_jamo() {
        assert_eq!(convert_korean_to_english("까싸"), "RkTk");
        assert_eq!(convert_korean_to_english("없"), "djqt");
        assert_eq!(convert_korean_to_english("닭"), "ekfr");
        assert_eq!(convert_korean_to_english("읽"), "dlfr");
        assert_eq!(convert_korean_to_english("삶"), "tkfa");
        assert_eq!(convert_korean_to_english("값"), "rkqt");
        assert_eq!(convert_korean_to_english("넓"), "sjfq");
        assert_eq!(convert_korean_to_english("앉"), "dksw");
        assert_eq!(convert_korean_to_english("않"), "dksg");
        assert_eq!(convert_korean_to_english("잃"), "dlfg");
        assert_eq!(convert_korean_to_english("핥"), "gkfx");
        assert_eq!(convert_korean_to_english("읊"), "dmfv");
        assert_eq!(convert_korean_to_english("ㅘㄳ"), "hkrt");
    }

    #[test]
    fn normalizes_canonical_jamo() {
        assert_eq!(
            convert_korean_to_english("\u{1100}\u{116A}\u{11AA}"),
            "rhkrt"
        );
    }

    #[test]
    fn detects_korean_ranges() {
        assert!(contains_korean("안녕하세요"));
        assert!(contains_korean("ㅍㅣ"));
        assert!(contains_korean("\u{1100}\u{1161}"));
        assert!(!contains_korean("hello"));
        assert!(!contains_korean("123!"));
        assert!(!contains_korean(""));
    }
}
