use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

/// 한국어 키보드로 잘못 입력된 명령어를 영어로 변환하는 매핑
static KOREAN_TO_ENGLISH_MAP: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
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

    // 겹받침 (compound final consonants)
    map.insert('ㄳ', "rt"); // ㄱ+ㅅ
    map.insert('ㄵ', "sw"); // ㄴ+ㅈ
    map.insert('ㄶ', "sg"); // ㄴ+ㅎ
    map.insert('ㄺ', "fr"); // ㄹ+ㄱ
    map.insert('ㄻ', "fa"); // ㄹ+ㅁ
    map.insert('ㄼ', "fq"); // ㄹ+ㅂ
    map.insert('ㄽ', "ft"); // ㄹ+ㅅ
    map.insert('ㄾ', "fx"); // ㄹ+ㅌ
    map.insert('ㄿ', "fv"); // ㄹ+ㅍ
    map.insert('ㅀ', "fg"); // ㄹ+ㅎ
    map.insert('ㅄ', "qt"); // ㅂ+ㅅ

    map
});

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

/// 한국어로 입력된 문자열을 영어로 변환
/// - 먼저 NFC 정규화를 수행하여 가능한 경우 완성형으로 결합
/// - 이후 음절은 자모로 분해, 단일 자모는 그대로 두고 매핑으로 변환
pub fn convert_korean_to_english(korean_input: &str) -> String {
    let map = &*KOREAN_TO_ENGLISH_MAP;

    // NFC 정규화로 NFD 입력을 최대한 완성형으로 결합
    let normalized: String = korean_input.nfc().collect();

    normalized
        .chars()
        .flat_map(decompose_hangul)
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

/// 두벌식 자판에서 Shift 여부를 알 수 없는 영문자 집합
/// Shift를 눌러도 같은 한글 자모가 입력되는 키들 (쌍자음/복합모음이 없는 키)
static AMBIGUOUS_ENGLISH_CHARS: LazyLock<HashSet<char>> = LazyLock::new(|| {
    [
        'y', 'u', 'i', // ㅛ, ㅕ, ㅑ
        'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', // ㅁ, ㄴ, ㅇ, ㄹ, ㅎ, ㅗ, ㅓ, ㅏ, ㅣ
        'z', 'x', 'c', 'v', 'b', 'n', 'm', // ㅋ, ㅌ, ㅊ, ㅍ, ㅠ, ㅜ, ㅡ
    ]
    .into_iter()
    .collect()
});

/// 영문자 하나에 대해 가능한 대소문자 후보를 반환
/// Shift 여부가 모호한 문자는 소문자와 대문자 모두 후보로 반환
fn char_variants(ch: char) -> Vec<char> {
    let ambiguous = &*AMBIGUOUS_ENGLISH_CHARS;
    if ambiguous.contains(&ch) {
        vec![ch, ch.to_ascii_uppercase()]
    } else {
        vec![ch]
    }
}

/// 한국어 문자열을 대소문자 후보를 포함한 패턴으로 변환
/// 각 위치는 가능한 문자들의 집합 (Vec<char>)
pub fn korean_to_pattern(korean_input: &str) -> Vec<Vec<char>> {
    let map = &*KOREAN_TO_ENGLISH_MAP;
    let normalized: String = korean_input.nfc().collect();

    normalized
        .chars()
        .flat_map(decompose_hangul)
        .flat_map(|jamo| {
            if let Some(out) = map.get(&jamo) {
                out.chars().map(char_variants).collect::<Vec<_>>()
            } else {
                vec![vec![jamo]]
            }
        })
        .collect()
}

/// 명령어가 한국어 패턴에 매칭되는지 확인
fn matches_korean_pattern(command: &str, pattern: &[Vec<char>]) -> bool {
    let chars: Vec<char> = command.chars().collect();
    if chars.len() != pattern.len() {
        return false;
    }
    chars
        .iter()
        .zip(pattern.iter())
        .all(|(ch, variants)| variants.contains(ch))
}

/// 한국어 입력에 대해 대소문자 후보를 고려하여 일치하는 명령어를 찾음
/// 기본 변환(소문자)이 있으면 우선 반환하고, 나머지 후보를 뒤에 추가
pub fn find_matching_commands(korean_input: &str, commands: &[&str]) -> Vec<String> {
    let pattern = korean_to_pattern(korean_input);
    let default_conversion = convert_korean_to_english(korean_input);

    let mut results = Vec::new();
    let mut has_default = false;

    for &cmd in commands {
        if matches_korean_pattern(cmd, &pattern) {
            if cmd == default_conversion {
                has_default = true;
            } else {
                results.push(cmd.to_string());
            }
        }
    }

    // 기본 변환(소문자)을 최우선으로 배치
    if has_default {
        results.insert(0, default_conversion);
    }

    results
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
    fn test_compound_jongsung() {
        // 겹받침이 포함된 음절들의 변환 테스트
        assert_eq!(convert_korean_to_english("없"), "djqt"); // ㅇ+ㅓ+ㅂㅅ
        assert_eq!(convert_korean_to_english("닭"), "ekfr"); // ㄷ+ㅏ+ㄹㄱ
        assert_eq!(convert_korean_to_english("읽"), "dlfr"); // ㅇ+ㅣ+ㄹㄱ
        assert_eq!(convert_korean_to_english("삶"), "tkfa"); // ㅅ+ㅏ+ㄹㅁ
        assert_eq!(convert_korean_to_english("값"), "rkqt"); // ㄱ+ㅏ+ㅂㅅ
        assert_eq!(convert_korean_to_english("넓"), "sjfq"); // ㄴ+ㅓ+ㄹㅂ
        assert_eq!(convert_korean_to_english("앉"), "dksw"); // ㅇ+ㅏ+ㄴㅈ
        assert_eq!(convert_korean_to_english("않"), "dksg"); // ㅇ+ㅏ+ㄴㅎ
        assert_eq!(convert_korean_to_english("잃"), "dlfg"); // ㅇ+ㅣ+ㄹㅎ
        assert_eq!(convert_korean_to_english("핥"), "gkfx"); // ㅎ+ㅏ+ㄹㅌ
        assert_eq!(convert_korean_to_english("읊"), "dmfv"); // ㅇ+ㅡ+ㄹㅍ
    }

    #[test]
    fn test_char_variants_ambiguous() {
        // Shift 여부가 모호한 문자: 소문자와 대문자 모두 반환
        assert_eq!(char_variants('l'), vec!['l', 'L']);
        assert_eq!(char_variants('a'), vec!['a', 'A']);
        assert_eq!(char_variants('m'), vec!['m', 'M']);
    }

    #[test]
    fn test_char_variants_unambiguous() {
        // Shift 여부가 명확한 문자: 해당 케이스만 반환
        assert_eq!(char_variants('q'), vec!['q']); // ㅂ→q, ㅃ→Q
        assert_eq!(char_variants('e'), vec!['e']); // ㄷ→e, ㄸ→E
        assert_eq!(char_variants('Q'), vec!['Q']); // ㅃ→Q
        assert_eq!(char_variants('E'), vec!['E']); // ㄸ→E
    }

    #[test]
    fn test_korean_to_pattern() {
        // ㅣ는 l/L 모두 가능
        let pattern = korean_to_pattern("ㅣ");
        assert_eq!(pattern, vec![vec!['l', 'L']]);

        // ㄷ은 e만 가능 (ㄸ→E이므로 명확)
        let pattern = korean_to_pattern("ㄷ");
        assert_eq!(pattern, vec![vec!['e']]);

        // ㄸ은 E만 가능 (명확한 대문자)
        let pattern = korean_to_pattern("ㄸ");
        assert_eq!(pattern, vec![vec!['E']]);

        // ㅣㅅ → l/L, t만 가능
        let pattern = korean_to_pattern("ㅣㅅ");
        assert_eq!(pattern, vec![vec!['l', 'L'], vec!['t']]);
    }

    #[test]
    fn test_matches_korean_pattern() {
        // ㅣ → l/L 패턴
        let pattern = korean_to_pattern("ㅣ");
        assert!(matches_korean_pattern("l", &pattern));
        assert!(matches_korean_pattern("L", &pattern));
        assert!(!matches_korean_pattern("k", &pattern));

        // ㅣㅅ → [l/L][t] 패턴
        let pattern = korean_to_pattern("ㅣㅅ");
        assert!(matches_korean_pattern("lt", &pattern));
        assert!(matches_korean_pattern("Lt", &pattern));
        assert!(!matches_korean_pattern("lT", &pattern)); // ㅅ→t는 명확 (ㅆ→T)
        assert!(!matches_korean_pattern("ls", &pattern));
    }

    #[test]
    fn test_find_matching_commands_exact() {
        // 기본 변환(소문자)이 명령어 목록에 있는 경우
        // ㅣ→l, ㅅ→t이므로 기본 변환은 "lt"
        let commands = vec!["ls", "lt", "git", "npm"];
        let results = find_matching_commands("ㅣㅅ", &commands);
        assert_eq!(results, vec!["lt"]); // "lt"가 기본 변환이므로 우선 반환
    }

    #[test]
    fn test_find_matching_commands_case_variant() {
        // ㅣ의 대문자 후보 (L)를 이용하여 매칭
        let commands = vec!["ls", "Lt", "git", "npm"];
        let results = find_matching_commands("ㅣㅅ", &commands);
        assert_eq!(results, vec!["Lt"]);
    }

    #[test]
    fn test_find_matching_commands_default_priority() {
        // 기본 변환과 대문자 후보가 모두 있는 경우, 기본 변환이 우선
        let commands = vec!["Lt", "lt", "git"];
        let results = find_matching_commands("ㅣㅅ", &commands);
        assert_eq!(results[0], "lt"); // 기본 변환이 첫 번째
        assert!(results.contains(&"Lt".to_string())); // 대문자 후보도 포함
    }

    #[test]
    fn test_find_matching_commands_no_match() {
        // 매칭되는 명령어가 없는 경우
        let commands = vec!["ls", "git", "npm"];
        let results = find_matching_commands("ㅣㅅ", &commands);
        assert!(results.is_empty());
    }

    #[test]
    fn test_find_matching_commands_unambiguous() {
        // Shift가 명확한 문자 (ㄷ→e, ㄸ→E)는 대소문자 후보 없음
        let commands = vec!["echo", "Echo"];
        // 'ㄷ'은 'e'로만 변환되므로 "echo"만 매칭 (ㄷ→e는 명확)
        // echo = e+c+h+o → ㄷ+쵀 (ㅊ+ㅙ = 쵀)
        let results = find_matching_commands("ㄷ쵀", &commands);
        // pattern: [e], [c/C], [h/H], [o]
        // "echo" → e∈[e]✓, c∈[c,C]✓, h∈[h,H]✓, o∈[o]✓ → match
        // "Echo" → E∈[e]✗ → no match
        assert_eq!(results, vec!["echo"]);
    }

    #[test]
    fn test_find_matching_commands_compound_vowel() {
        // 복합모음이 포함된 경우 (ㅘ→hk, 둘 다 ambiguous)
        let pattern = korean_to_pattern("ㅘ");
        assert_eq!(
            pattern,
            vec![vec!['h', 'H'], vec!['k', 'K']]
        );
        assert!(matches_korean_pattern("hk", &pattern));
        assert!(matches_korean_pattern("HK", &pattern));
        assert!(matches_korean_pattern("Hk", &pattern));
        assert!(matches_korean_pattern("hK", &pattern));
    }
}
