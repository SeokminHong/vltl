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
        if jongsung_idx > 0 {
            if let Some(jong) = jongsung.get(jongsung_idx as usize) {
                for c in jong.chars() {
                    result.push(c);
                }
            }
        }
        result
    } else {
        // 완성형이 아니면 그대로 반환
        vec![ch]
    }
}

/// 한국어로 입력된 문자열을 영어로 변환
/// - 먼저 NFC 정규화를 수행하여 가능한 경우 완성형으로 결합
/// - 이후 음절은 자모로 분해, 단일 자모는 그대로 두고 매핑으로 변환
pub fn convert_korean_to_english(korean_input: &str) -> String {
    let map = get_korean_to_english_map();

    // NFC 정규화로 NFD 입력을 최대한 완성형으로 결합
    let normalized: String = korean_input.nfc().collect();

    normalized
        .chars()
        .flat_map(|c| decompose_hangul(c))
        .flat_map(|jamo| {
            // 매핑이 있으면 그 문자열을, 없으면 원문 글자를 사용
            if let Some(out) = map.get(&jamo) {
                out.chars().collect::<Vec<char>>()
            } else {
                vec![jamo]
            }
        })
        .collect()
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
}
