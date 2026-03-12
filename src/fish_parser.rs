/// Fish shell command line parser for extracting program names.
///
/// Parses a Fish command line and extracts the first non-assignment token
/// from each command, properly handling Fish's quoting and escaping rules.

/// Extract program names from a Fish shell command line.
///
/// For each command separated by operators (`&&`, `||`, `|`, `;`, `&`, newlines),
/// returns the first non-assignment token (the program name).
///
/// Properly handles:
/// - Fish single quotes: `'...'` with `\'` and `\\` escapes
/// - Fish double quotes: `"..."` with `\"`, `\\`, `\$`, `\n` escapes
/// - Backslash escaping outside quotes
/// - Command substitution: `(...)` (nested correctly)
/// - Environment variable assignments: `KEY=VALUE` prefixes are skipped
/// - Comments: `#` to end of line
pub fn extract_program_names(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut pos = 0;
    let mut result = Vec::new();
    let mut looking_for_program = true;

    while pos < len {
        let c = chars[pos];

        // Newlines are command separators
        if c == '\n' {
            looking_for_program = true;
            pos += 1;
            continue;
        }

        // Skip other whitespace
        if c.is_ascii_whitespace() {
            pos += 1;
            continue;
        }

        // Comments: skip to end of line
        if c == '#' {
            while pos < len && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        // Command separators
        if c == ';' {
            looking_for_program = true;
            pos += 1;
            continue;
        }
        if c == '&' {
            if pos + 1 < len && chars[pos + 1] == '&' {
                pos += 2; // &&
            } else {
                pos += 1; // & (background)
            }
            looking_for_program = true;
            continue;
        }
        if c == '|' {
            if pos + 1 < len && chars[pos + 1] == '|' {
                pos += 2; // ||
            } else {
                pos += 1; // | (pipe)
            }
            looking_for_program = true;
            continue;
        }

        // Read a token
        let (word, has_unquoted_eq, new_pos) = read_token(&chars, pos);
        pos = new_pos;

        if looking_for_program {
            if has_unquoted_eq {
                // Environment variable assignment, skip
                continue;
            }
            if !word.is_empty() {
                result.push(word);
                looking_for_program = false;
            }
        }
    }

    result
}

/// Read a single token from the character stream.
///
/// Returns `(unquoted_word, has_unquoted_equals, new_position)`.
fn read_token(chars: &[char], start: usize) -> (String, bool, usize) {
    let len = chars.len();
    let mut pos = start;
    let mut word = String::new();
    let mut has_unquoted_equals = false;

    while pos < len {
        let c = chars[pos];

        // Token boundary: whitespace or operator characters
        if c.is_ascii_whitespace() || c == ';' || c == '#' || c == '&' || c == '|' {
            break;
        }

        match c {
            // Fish single quotes: only \' and \\ are escape sequences
            '\'' => {
                pos += 1;
                while pos < len && chars[pos] != '\'' {
                    if chars[pos] == '\\'
                        && pos + 1 < len
                        && matches!(chars[pos + 1], '\'' | '\\')
                    {
                        word.push(chars[pos + 1]);
                        pos += 2;
                    } else {
                        word.push(chars[pos]);
                        pos += 1;
                    }
                }
                if pos < len {
                    pos += 1; // skip closing '
                }
            }

            // Fish double quotes: \", \\, \$, \n are escape sequences
            '"' => {
                pos += 1;
                while pos < len && chars[pos] != '"' {
                    if chars[pos] == '\\' && pos + 1 < len {
                        match chars[pos + 1] {
                            '"' | '\\' | '$' => {
                                word.push(chars[pos + 1]);
                                pos += 2;
                            }
                            'n' => {
                                word.push('\n');
                                pos += 2;
                            }
                            _ => {
                                word.push('\\');
                                word.push(chars[pos + 1]);
                                pos += 2;
                            }
                        }
                    } else {
                        word.push(chars[pos]);
                        pos += 1;
                    }
                }
                if pos < len {
                    pos += 1; // skip closing "
                }
            }

            // Command substitution: skip to matching ), tracking nesting
            '(' => {
                pos += 1;
                let new_pos = skip_command_substitution(chars, pos);
                // Include raw content (we can't evaluate the substitution)
                for &ch in &chars[start..new_pos] {
                    word.push(ch);
                }
                pos = new_pos;
            }

            // Backslash escape outside quotes
            '\\' if pos + 1 < len => {
                word.push(chars[pos + 1]);
                pos += 2;
            }

            // Regular character
            _ => {
                if c == '=' {
                    has_unquoted_equals = true;
                }
                word.push(c);
                pos += 1;
            }
        }
    }

    (word, has_unquoted_equals, pos)
}

/// Skip past a command substitution `(...)`, handling nesting and quoting.
///
/// `start` should be the position right after the opening `(`.
/// Returns the position right after the closing `)`.
fn skip_command_substitution(chars: &[char], start: usize) -> usize {
    let len = chars.len();
    let mut pos = start;
    let mut depth: u32 = 1;

    while pos < len && depth > 0 {
        match chars[pos] {
            '(' => {
                depth += 1;
                pos += 1;
            }
            ')' => {
                depth -= 1;
                pos += 1;
            }
            '\'' => {
                pos += 1;
                while pos < len && chars[pos] != '\'' {
                    if chars[pos] == '\\'
                        && pos + 1 < len
                        && matches!(chars[pos + 1], '\'' | '\\')
                    {
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                }
                if pos < len {
                    pos += 1;
                }
            }
            '"' => {
                pos += 1;
                while pos < len && chars[pos] != '"' {
                    if chars[pos] == '\\' && pos + 1 < len {
                        pos += 2;
                    } else {
                        pos += 1;
                    }
                }
                if pos < len {
                    pos += 1;
                }
            }
            '\\' if pos + 1 < len => {
                pos += 2;
            }
            _ => {
                pos += 1;
            }
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        assert_eq!(extract_program_names("ls -la"), vec!["ls"]);
    }

    #[test]
    fn test_pipe() {
        assert_eq!(extract_program_names("echo hello | cat"), vec!["echo", "cat"]);
    }

    #[test]
    fn test_and_operator() {
        assert_eq!(
            extract_program_names("echo hello && ls"),
            vec!["echo", "ls"]
        );
    }

    #[test]
    fn test_or_operator() {
        assert_eq!(
            extract_program_names("false || echo fallback"),
            vec!["false", "echo"]
        );
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(
            extract_program_names("echo hello; ls"),
            vec!["echo", "ls"]
        );
    }

    #[test]
    fn test_background() {
        assert_eq!(extract_program_names("echo hello &"), vec!["echo"]);
    }

    #[test]
    fn test_env_var_assignment() {
        assert_eq!(
            extract_program_names("VAR=value echo hello"),
            vec!["echo"]
        );
    }

    #[test]
    fn test_multiple_env_var_assignments() {
        assert_eq!(
            extract_program_names("VAR1=a VAR2=b echo hello"),
            vec!["echo"]
        );
    }

    #[test]
    fn test_korean_env_var() {
        assert_eq!(
            extract_program_names("변수=all yarn lint"),
            vec!["yarn"]
        );
    }

    #[test]
    fn test_quoted_equals_not_assignment() {
        // '=' inside quotes should NOT be treated as assignment
        assert_eq!(
            extract_program_names("'VAR=value' arg"),
            vec!["VAR=value"]
        );
    }

    #[test]
    fn test_quoted_program_name() {
        assert_eq!(extract_program_names("'ls' -la"), vec!["ls"]);
    }

    #[test]
    fn test_korean_command() {
        assert_eq!(extract_program_names("ㅣㄴ -la"), vec!["ㅣㄴ"]);
    }

    #[test]
    fn test_escaped_quote_in_arg() {
        // echo '변수=all a\'b' — the program is "echo"
        assert_eq!(
            extract_program_names("echo '변수=all a\\'b'"),
            vec!["echo"]
        );
    }

    #[test]
    fn test_compound_with_env_var() {
        assert_eq!(
            extract_program_names("VAR=x echo && VAR=y ls"),
            vec!["echo", "ls"]
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            extract_program_names("echo hello # this is a comment"),
            vec!["echo"]
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(extract_program_names(""), Vec::<String>::new());
    }

    #[test]
    fn test_only_env_var() {
        assert_eq!(extract_program_names("VAR=value"), Vec::<String>::new());
    }

    #[test]
    fn test_newline_separator() {
        assert_eq!(
            extract_program_names("echo hello\nls"),
            vec!["echo", "ls"]
        );
    }

    #[test]
    fn test_command_substitution_with_operators() {
        // Operators inside (...) should NOT split commands
        assert_eq!(
            extract_program_names("echo (cmd1 && cmd2)"),
            vec!["echo"]
        );
    }

    #[test]
    fn test_multiple_operators() {
        assert_eq!(
            extract_program_names("echo a; ls -l && cat file || grep pattern | head"),
            vec!["echo", "ls", "cat", "grep", "head"]
        );
    }

    #[test]
    fn test_double_quoted_pipe() {
        // | inside double quotes should NOT be treated as pipe
        assert_eq!(
            extract_program_names("echo \"hello | world\""),
            vec!["echo"]
        );
    }

    #[test]
    fn test_backslash_escape() {
        assert_eq!(extract_program_names("echo hello\\ world"), vec!["echo"]);
    }

    #[test]
    fn test_env_var_in_compound() {
        assert_eq!(
            extract_program_names("변수=all echo hello && 변수2=test ls"),
            vec!["echo", "ls"]
        );
    }

    #[test]
    fn test_only_whitespace() {
        assert_eq!(extract_program_names("   "), Vec::<String>::new());
    }

    #[test]
    fn test_only_comment() {
        assert_eq!(
            extract_program_names("# just a comment"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_double_quote_escape_sequences() {
        // echo "hello\"world" — program is echo
        assert_eq!(
            extract_program_names("echo \"hello\\\"world\""),
            vec!["echo"]
        );
    }

    #[test]
    fn test_complex_edge_case() {
        // The motivating edge case from the issue:
        // echo '변수=all a\'b'
        assert_eq!(
            extract_program_names("echo '변수=all a\\'b'"),
            vec!["echo"]
        );
    }
}
