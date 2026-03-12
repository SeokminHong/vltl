/// Fish shell command line parser for extracting program names.
///
/// Uses [tree-sitter-fish](https://github.com/ram02z/tree-sitter-fish) to parse
/// Fish command lines and extract the program name from each command.

use tree_sitter::{Node, Parser};

/// Extract program names from a Fish shell command line.
///
/// For each command separated by operators (`&&`, `||`, `|`, `;`, `&`, newlines),
/// returns the first non-assignment token (the program name).
///
/// Uses the tree-sitter-fish grammar to correctly handle:
/// - Quoting: single quotes, double quotes, escape sequences
/// - Command separators: `&&`, `||`, `|`, `;`, `&`, newlines
/// - Command substitution: `(...)` (nested correctly)
/// - Environment variable assignments: `KEY=VALUE` prefixes are skipped
/// - Comments
pub fn extract_program_names(input: &str) -> Vec<String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_fish::language())
        .expect("Error loading fish grammar");

    let Some(tree) = parser.parse(input, None) else {
        return Vec::new();
    };

    let mut names = Vec::new();
    collect_command_names(&tree.root_node(), input.as_bytes(), &mut names);
    names
}

/// Recursively walk the AST to find `command` nodes and extract their program names.
///
/// When tree-sitter-fish cannot fully parse a command (e.g. a single word with
/// no arguments), the grammar wraps the tokens in an `ERROR` node instead of a
/// `command` node.  In that case we fall back to treating the first
/// non-assignment `word` child of the `ERROR` node as the program name.
fn collect_command_names(node: &Node, source: &[u8], names: &mut Vec<String>) {
    if node.kind() == "command" {
        if let Some(name) = get_program_name(node, source) {
            names.push(name);
        }
        return;
    }

    // Handle ERROR nodes that contain bare words but no command children.
    if node.is_error() {
        let has_command_child = {
            let mut cursor = node.walk();
            let mut found = false;
            if cursor.goto_first_child() {
                loop {
                    if cursor.node().kind() == "command" {
                        found = true;
                        break;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            found
        };

        if has_command_child {
            // Recurse normally to pick up the nested command(s).
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    collect_command_names(&cursor.node(), source, names);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        } else {
            // No command child — treat the first non-assignment word as the
            // program name.
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    let child = cursor.node();
                    if child.kind() == "word" && !is_assignment_word(&child, source) {
                        names.push(node_text_unquoted(&child, source));
                        break;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
        return;
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_command_names(&cursor.node(), source, names);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Check whether a node is a plain `word` containing `=` (i.e., a variable assignment).
fn is_assignment_word(node: &Node, source: &[u8]) -> bool {
    if node.kind() != "word" {
        return false;
    }
    source[node.byte_range()].contains(&b'=')
}

/// Extract the text content of a node, stripping surrounding quotes if it is a
/// `single_quote_string` or `double_quote_string`.
fn node_text_unquoted(node: &Node, source: &[u8]) -> String {
    let text = String::from_utf8_lossy(&source[node.byte_range()]).to_string();
    match node.kind() {
        "single_quote_string" | "double_quote_string" => {
            if text.len() >= 2 {
                text[1..text.len() - 1].to_string()
            } else {
                text
            }
        }
        _ => text,
    }
}

/// Get the program name from a `command` node, skipping any leading variable assignments.
fn get_program_name(command_node: &Node, source: &[u8]) -> Option<String> {
    // Check the `name` field (first token of the command)
    if let Some(name_node) = command_node.child_by_field_name("name") {
        if !is_assignment_word(&name_node, source) {
            return Some(node_text_unquoted(&name_node, source));
        }
    }
    // The `name` field was a variable assignment — look through `argument` fields
    let mut cursor = command_node.walk();
    if cursor.goto_first_child() {
        loop {
            if cursor.field_name() == Some("argument") {
                let child = cursor.node();
                if !is_assignment_word(&child, source) {
                    return Some(node_text_unquoted(&child, source));
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
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

    #[test]
    fn test_single_korean_command_no_args() {
        // Single Korean word without arguments should still be extracted as a
        // program name.  (The actual Korean→English conversion — e.g.
        // ㅛㅁ구 → yarn — is handled by the converter module.)
        assert_eq!(extract_program_names("ㅛㅁ구"), vec!["ㅛㅁ구"]);
    }
}
