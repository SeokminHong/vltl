use std::collections::HashMap;

/// 한국어 키보드로 잘못 입력된 명령어를 영어로 변환하는 매핑
fn get_korean_to_english_map() -> HashMap<char, char> {
    let mut map = HashMap::new();
    
    // 자음
    map.insert('ㅂ', 'q');  // ㅂ -> q
    map.insert('ㅈ', 'w');  // ㅈ -> w
    map.insert('ㄷ', 'e');  // ㄷ -> e
    map.insert('ㄱ', 'r');  // ㄱ -> r
    map.insert('ㅅ', 't');  // ㅅ -> t
    map.insert('ㅛ', 'y');  // ㅛ -> y
    map.insert('ㅕ', 'u');  // ㅕ -> u
    map.insert('ㅑ', 'i');  // ㅑ -> i
    map.insert('ㅐ', 'o');  // ㅐ -> o
    map.insert('ㅔ', 'p');  // ㅔ -> p
    
    map.insert('ㅁ', 'a');  // ㅁ -> a
    map.insert('ㄴ', 's');  // ㄴ -> s
    map.insert('ㅇ', 'd');  // ㅇ -> d
    map.insert('ㄹ', 'f');  // ㄹ -> f
    map.insert('ㅎ', 'g');  // ㅎ -> g
    map.insert('ㅗ', 'h');  // ㅗ -> h
    map.insert('ㅓ', 'j');  // ㅓ -> j
    map.insert('ㅏ', 'k');  // ㅏ -> k
    map.insert('ㅣ', 'l');  // ㅣ -> l
    
    map.insert('ㅋ', 'z');  // ㅋ -> z
    map.insert('ㅌ', 'x');  // ㅌ -> x
    map.insert('ㅊ', 'c');  // ㅊ -> c
    map.insert('ㅍ', 'v');  // ㅍ -> v
    map.insert('ㅠ', 'b');  // ㅠ -> b
    map.insert('ㅜ', 'n');  // ㅜ -> n
    map.insert('ㅡ', 'm');  // ㅡ -> m
    
    // 쌍자음
    map.insert('ㅃ', 'Q');  // ㅃ -> Q
    map.insert('ㅉ', 'W');  // ㅉ -> W
    map.insert('ㄸ', 'E');  // ㄸ -> E
    map.insert('ㄲ', 'R');  // ㄲ -> R
    map.insert('ㅆ', 'T');  // ㅆ -> T
    
    // 복합모음
    map.insert('ㅒ', 'O');  // ㅒ -> O
    map.insert('ㅖ', 'P');  // ㅖ -> P
    
    map
}

/// 한글 완성형 문자를 자모로 분해
fn decompose_hangul(ch: char) -> Vec<char> {
    let code = ch as u32;
    
    // 한글 음절 범위 (가-힣: U+AC00 - U+D7A3)
    if code >= 0xAC00 && code <= 0xD7A3 {
        let base = code - 0xAC00;
        
        // 초성, 중성, 종성 인덱스 계산
        let chosung_idx = base / (21 * 28);
        let jungsung_idx = (base % (21 * 28)) / 28;
        let jongsung_idx = base % 28;
        
        // 초성 테이블
        let chosung = ['ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
        
        // 중성 테이블
        let jungsung = ['ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'];
        
        // 종성 테이블 (첫 번째는 빈 종성)
        let jongsung = ["", "ㄱ", "ㄲ", "ㄳ", "ㄴ", "ㄵ", "ㄶ", "ㄷ", "ㄹ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅁ", "ㅂ", "ㅄ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ"];
        
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
        vec![ch]
    }
}

/// 한국어로 입력된 문자열을 영어로 변환
pub fn convert_korean_to_english(korean_input: &str) -> String {
    let map = get_korean_to_english_map();
    
    korean_input
        .chars()
        .flat_map(|c| decompose_hangul(c))
        .map(|c| map.get(&c).copied().unwrap_or(c))
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
}
