use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

/// 한국어 키보드로 잘못 입력된 명령어를 영어로 변환하는 매핑
fn get_korean_to_english_map() -> HashMap<char, &'static str> {
    let mut map: HashMap<char, &'static str> = HashMap::new();

    // 자음
    map.insert('ㅂ', "q");
    map.insert('ㅈ', "w");
    map.insert('ㄷ', "e");
    map.insert('ㄱ', "r");
    map.insert('ㅅ', "t");
    map.insert('ㅛ', "y");
    map.insert('ㅕ', "u");
    map.insert('ㅑ', "i");
    map.insert('ㅐ', "o");
    map.insert('ㅔ', "p");

    map.insert('ㅁ', "a");
    map.insert('ㄴ', "s");
    map.insert('ㅇ', "d");
    map.insert('ㄹ', "f");
    map.insert('ㅎ', "g");
    map.insert('ㅗ', "h");
    map.insert('ㅓ', "j");
    map.insert('ㅏ', "k");
    map.insert('ㅣ', "l");

    map.insert('ㅋ', "z");
    map.insert('ㅌ', "x");
    map.insert('ㅊ', "c");
    map.insert('ㅍ', "v");
    map.insert('ㅠ', "b");
    map.insert('ㅜ', "n");
    map.insert('ㅡ', "m");

    // 쌍자음
    map.insert('ㅃ', "Q");
    map.insert('ㅉ', "W");
    map.insert('ㄸ', "E");
    map.insert('ㄲ', "R");
    map.insert('ㅆ', "T");

    // 복합모음: 두벌식 키 시퀀스에 맞춰 2글자 이상으로 매핑
    map.insert('ㅒ', "O"); // ㅒ (yae) → 보통 Shift+o로 들어온 호환자모
    map.insert('ㅖ', "P"); // ㅖ (ye)  → 보통 Shift+p
    map.insert('ㅘ', "hk"); // ㅗ+ㅏ
    map.insert('ㅙ', "ho"); // ㅗ+ㅐ
    map.insert('ㅚ', "hl"); // ㅗ+ㅣ
    map.insert('ㅝ', "nj"); // ㅜ+ㅓ
    map.insert('ㅞ', "np"); // ㅜ+ㅔ
    map.insert('ㅟ', "nl"); // ㅜ+ㅣ
    map.insert('ㅢ', "ml"); // ㅡ+ㅣ

    map
}

/// 한글 완성형 문자를 자모로 분해
fn decompose_hangul(ch: char) -> Vec<char> {
    let code = ch as u32;

    // 한글 음절 범위 (가-힣: U+AC00 - U+D7A3)
    if (0xAC00..=0xD7A3).contains(&code) {
        let base = code - 0xAC00;

        // 초성, 중성, 종성 인덱스 계산
        let chosung_idx = base / (21 * 28);
        let jungsung_idx = (base % (21 * 28)) / 28;
        let jongsung_idx = base % 28;

        // 초성 테이블
        let chosung = [
            'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ',
            'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
        ];

        // 중성 테이블
        let jungsung = [
            'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ',
            'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
        ];

        // 종성 테이블 (첫 번째는 빈 종성)
        let jongsung = [
            "", "ㄱ", "ㄲ", "ㄳ", "ㄴ", "ㄵ", "ㄶ", "ㄷ", "ㄹ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ",
            "ㅀ", "ㅁ", "ㅂ", "ㅄ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ",
        ];

        let mut result = Vec::new();

        if let Some(cho) = chosung.get(chosung_idx as usize) {
            result.push(*cho);
        }
        if let Some(jung) = jungsung.get(jungsung_idx as usize) {
            result.push(*jung);
        }
        if jongsung_idx > 0
            && let Some(jong) = jongsung.get(jongsung_idx as usize)
        {
            for c in jong.chars() {
                result.push(c);
            }
        }
        result
    } else {
        // 완성형이 아니면 그대로 반환
        vec![ch]
    }
}

/// 문자열에 한국어 문자가 포함되어 있는지 확인
/// - 한글 완성형 음절 (가-힣: U+AC00 - U+D7A3)
/// - 한글 자모 (ㄱ-ㅎ, ㅏ-ㅣ: U+3131 - U+318E)
pub fn contains_korean(input: &str) -> bool {
    input.chars().any(|c| {
        let code = c as u32;
        // 한글 완성형 음절 (가-힣)
        (0xAC00..=0xD7A3).contains(&code)
            // 한글 자모 (ㄱ-ㅎ, ㅏ-ㅣ)
            || (0x3131..=0x318E).contains(&code)
    })
}

fn is_shift_ambiguous_jamo(jamo: char) -> bool {
    matches!(
        jamo,
        'ㅛ' | 'ㅕ'
            | 'ㅑ'
            | 'ㅁ'
            | 'ㄴ'
            | 'ㅇ'
            | 'ㄹ'
            | 'ㅎ'
            | 'ㅗ'
            | 'ㅓ'
            | 'ㅏ'
            | 'ㅣ'
            | 'ㅋ'
            | 'ㅌ'
            | 'ㅊ'
            | 'ㅍ'
            | 'ㅠ'
            | 'ㅜ'
            | 'ㅡ'
            | 'ㅘ'
            | 'ㅙ'
            | 'ㅚ'
            | 'ㅝ'
            | 'ㅞ'
            | 'ㅟ'
            | 'ㅢ'
    )
}

fn english_variants_for_jamo(jamo: char, map: &HashMap<char, &'static str>) -> Vec<String> {
    let Some(out) = map.get(&jamo) else {
        return vec![jamo.to_string()];
    };

    if !is_shift_ambiguous_jamo(jamo) {
        return vec![(*out).to_string()];
    }

    out.chars().fold(vec![String::new()], |variants, ch| {
        variants
            .into_iter()
            .flat_map(|prefix| {
                let mut next = Vec::with_capacity(2);
                next.push(format!("{prefix}{ch}"));
                if ch.is_ascii_lowercase() {
                    next.push(format!("{prefix}{}", ch.to_ascii_uppercase()));
                }
                next
            })
            .collect()
    })
}

/// 한국어로 입력된 문자열을 영어 후보들로 변환
/// - Shift 여부를 알 수 없는 음소에 대해서는 대소문자 후보를 모두 생성
/// - 현재 매핑에서 한 자모는 최대 2개의 영문 키로만 확장되므로, 자모 하나당 후보 수는 최대 4개다.
pub fn convert_korean_to_english_candidates(korean_input: &str) -> Vec<String> {
    let map = get_korean_to_english_map();

    let normalized: String = korean_input.nfc().collect();

    normalized
        .chars()
        .flat_map(decompose_hangul)
        .fold(vec![String::new()], |candidates, jamo| {
            let variants = english_variants_for_jamo(jamo, &map);
            candidates
                .into_iter()
                .flat_map(|prefix| {
                    variants
                        .iter()
                        .map(move |variant| format!("{prefix}{variant}"))
                })
                .collect()
        })
}

/// 한국어로 입력된 문자열을 영어로 변환
/// - 먼저 NFC 정규화를 수행하여 가능한 경우 완성형으로 결합
/// - 이후 음절은 자모로 분해, 단일 자모는 그대로 두고 매핑으로 변환
/// - Shift 여부가 모호한 경우에는 후보들 중 첫 번째(기존과 동일한 기본 소문자 경로)를 반환
pub fn convert_korean_to_english(korean_input: &str) -> String {
    convert_korean_to_english_candidates(korean_input)
        .into_iter()
        .next()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_conversion() {
        assert_eq!(convert_korean_to_english("피"), "vl");
        assert_eq!(convert_korean_to_english("며"), "au");
        assert_eq!(convert_korean_to_english("내"), "so");
    }

    #[test]
    fn test_decompose_hangul() {
        let result = decompose_hangul('며');
        assert_eq!(result, vec!['ㅁ', 'ㅕ']);

        let result = decompose_hangul('피');
        assert_eq!(result, vec!['ㅍ', 'ㅣ']);
    }

    #[test]
    fn test_non_completed() {
        assert_eq!(convert_korean_to_english("ㅍㅣ"), "vl");
        assert_eq!(convert_korean_to_english("ㅔㅞㅡ"), "pnpm");
        assert_eq!(convert_korean_to_english("ㅛㅁ구"), "yarn");
        assert_eq!(convert_korean_to_english("ㅎㄱ데"), "grep");
    }

    #[test]
    fn test_contains_korean() {
        // 한글 완성형
        assert!(contains_korean("피"));
        assert!(contains_korean("며"));
        assert!(contains_korean("내"));
        assert!(contains_korean("안녕하세요"));

        // 한글 자모
        assert!(contains_korean("ㅍㅣ"));
        assert!(contains_korean("ㅔㅞㅡ"));
        assert!(contains_korean("ㅛㅁ구"));

        // 영문
        assert!(!contains_korean("ls"));
        assert!(!contains_korean("npm"));
        assert!(!contains_korean("hello"));
        assert!(!contains_korean("nonexistent"));

        // 혼합
        assert!(contains_korean("ls안녕"));
        assert!(contains_korean("helloㅎㅎ"));

        // 기타
        assert!(!contains_korean(""));
        assert!(!contains_korean("123"));
        assert!(!contains_korean("!@#$"));
    }

    #[test]
    fn test_shift_ambiguous_candidates() {
        assert_eq!(convert_korean_to_english_candidates("ㅣ"), vec!["l", "L"]);
        assert_eq!(
            convert_korean_to_english_candidates("ㅢ"),
            vec!["ml", "mL", "Ml", "ML"]
        );
        assert_eq!(
            convert_korean_to_english_candidates("햣"),
            vec!["git", "gIt", "Git", "GIt"]
        );
    }

    #[test]
    fn test_fixed_shift_candidates() {
        assert_eq!(convert_korean_to_english_candidates("ㄸ"), vec!["E"]);
        assert_eq!(convert_korean_to_english_candidates("ㅖ"), vec!["P"]);
    }
}
