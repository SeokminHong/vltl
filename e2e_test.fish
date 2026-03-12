#!/usr/bin/env fish

# E2E test script for vltl
# This script tests the abbr-based fish integration

set -g test_failed 0
set -g test_passed 0

function print_test_result
    set -l test_name $argv[1]
    set -l result $argv[2]

    if test $result -eq 0
        echo "✓ PASS: $test_name"
        set -g test_passed (math $test_passed + 1)
    else
        echo "✗ FAIL: $test_name"
        set -g test_failed (math $test_failed + 1)
    end
end

function test_function_definitions
    echo ""
    echo "Testing function definitions after sourcing init..."

    # Source the init script
    vltl init | source

    # Test: __vltl_convert_and_expand is defined
    if type -q __vltl_convert_and_expand
        print_test_result "function __vltl_convert_and_expand is defined" 0
    else
        print_test_result "function __vltl_convert_and_expand is defined" 1
    end

    # Test: __vltl_auto_register_abbr is defined
    if type -q __vltl_auto_register_abbr
        print_test_result "function __vltl_auto_register_abbr is defined" 0
    else
        print_test_result "function __vltl_auto_register_abbr is defined" 1
    end

    # Test: __vltl_abbr_space is defined
    if type -q __vltl_abbr_space
        print_test_result "function __vltl_abbr_space is defined" 0
    else
        print_test_result "function __vltl_abbr_space is defined" 1
    end

    # Test: __vltl_abbr_enter is defined
    if type -q __vltl_abbr_enter
        print_test_result "function __vltl_abbr_enter is defined" 0
    else
        print_test_result "function __vltl_abbr_enter is defined" 1
    end

    # Test: __vltl_abbr_semicolon is defined
    if type -q __vltl_abbr_semicolon
        print_test_result "function __vltl_abbr_semicolon is defined" 0
    else
        print_test_result "function __vltl_abbr_semicolon is defined" 1
    end

    # Test: old alias-based functions should NOT be defined
    if not type -q __vltl
        print_test_result "old __vltl preexec function is not defined" 0
    else
        print_test_result "old __vltl preexec function is not defined" 1
    end

    if not type -q __vltl_check
        print_test_result "old __vltl_check helper is not defined" 0
    else
        print_test_result "old __vltl_check helper is not defined" 1
    end
end

function test_convert_command
    echo ""
    echo "Testing vltl convert command..."

    # Test: Korean syllable conversion
    set -l result (vltl convert "햣")
    if test "$result" = git
        print_test_result "convert: 햣 -> git" 0
    else
        print_test_result "convert: 햣 -> git (got: $result)" 1
    end

    # Test: Korean jamo conversion
    set -l result (vltl convert "ㅣ")
    if test "$result" = l
        print_test_result "convert: ㅣ -> l" 0
    else
        print_test_result "convert: ㅣ -> l (got: $result)" 1
    end

    # Test: Multi-character jamo conversion
    set -l result (vltl convert "ㅔㅞㅡ")
    if test "$result" = pnpm
        print_test_result "convert: ㅔㅞㅡ -> pnpm" 0
    else
        print_test_result "convert: ㅔㅞㅡ -> pnpm (got: $result)" 1
    end

    # Test: Mixed syllable conversion
    set -l result (vltl convert "ㅛㅁ구")
    if test "$result" = yarn
        print_test_result "convert: ㅛㅁ구 -> yarn" 0
    else
        print_test_result "convert: ㅛㅁ구 -> yarn (got: $result)" 1
    end
end

function test_has_korean_command
    echo ""
    echo "Testing vltl has-korean command..."

    # Test: Korean syllable detected
    if vltl has-korean "햣"
        print_test_result "has-korean: detects Korean syllable" 0
    else
        print_test_result "has-korean: detects Korean syllable" 1
    end

    # Test: Korean jamo detected
    if vltl has-korean "ㅣ"
        print_test_result "has-korean: detects Korean jamo" 0
    else
        print_test_result "has-korean: detects Korean jamo" 1
    end

    # Test: English not detected as Korean
    if not vltl has-korean "ls"
        print_test_result "has-korean: English not detected as Korean" 0
    else
        print_test_result "has-korean: English not detected as Korean" 1
    end

    # Test: Empty string not detected as Korean
    if not vltl has-korean ""
        print_test_result "has-korean: empty string not detected as Korean" 0
    else
        print_test_result "has-korean: empty string not detected as Korean" 1
    end
end

function test_auto_register_abbr
    echo ""
    echo "Testing abbr auto-registration..."

    # Source the init script
    vltl init | source

    # Setup: register an English-trigger abbr
    abbr -a -- testcmd_L 'echo expanded_L'

    # Test: auto-register Korean trigger for existing English abbr
    __vltl_auto_register_abbr "ㅣ" testcmd_L

    if abbr -q -- "ㅣ"
        print_test_result "auto-register: Korean trigger abbr is created" 0
    else
        print_test_result "auto-register: Korean trigger abbr is created" 1
    end

    # Cleanup
    abbr -e -- testcmd_L
    abbr -e -- "ㅣ"
end

function test_auto_register_preserves_options
    echo ""
    echo "Testing abbr auto-registration preserves options..."

    # Source the init script
    vltl init | source

    # Setup: register an English-trigger abbr with --position anywhere
    abbr -a --position anywhere -- testcmd_PA 'echo anywhere_test'

    # Auto-register Korean trigger
    __vltl_auto_register_abbr "ㅔㅁ" testcmd_PA

    if abbr -q -- "ㅔㅁ"
        print_test_result "auto-register: preserves options (abbr created)" 0

        # Check that the definition includes --position anywhere
        set -l def (abbr --show | string match -- "*ㅔㅁ*")
        if string match -q "*--position anywhere*" -- $def
            print_test_result "auto-register: --position anywhere is preserved" 0
        else
            print_test_result "auto-register: --position anywhere is preserved" 1
        end
    else
        print_test_result "auto-register: preserves options (abbr created)" 1
    end

    # Cleanup
    abbr -e -- testcmd_PA
    abbr -e -- "ㅔㅁ"
end

function test_auto_register_no_duplicate
    echo ""
    echo "Testing abbr auto-registration does not duplicate..."

    # Source the init script
    vltl init | source

    # Setup: register English-trigger abbr and manually create Korean one
    abbr -a -- testcmd_ND 'echo no_dup'
    abbr -a -- "ㄴㄷ" 'echo already_exists'

    # Try to auto-register - should not overwrite
    __vltl_auto_register_abbr "ㄴㄷ" testcmd_ND

    # Verify the existing Korean abbr is unchanged
    set -l def (abbr --show | string match -- "*ㄴㄷ*")
    if string match -q "*already_exists*" -- $def
        print_test_result "auto-register: does not overwrite existing Korean abbr" 0
    else
        print_test_result "auto-register: does not overwrite existing Korean abbr" 1
    end

    # Cleanup
    abbr -e -- testcmd_ND
    abbr -e -- "ㄴㄷ"
end

function test_auto_register_with_set_cursor
    echo ""
    echo "Testing abbr auto-registration with --set-cursor..."

    # Source the init script
    vltl init | source

    # Setup: register abbr with --set-cursor
    abbr -a --position anywhere --set-cursor -- testcmd_SC '% | less'

    # Auto-register Korean trigger
    __vltl_auto_register_abbr "ㅅㅊ" testcmd_SC

    if abbr -q -- "ㅅㅊ"
        set -l def (abbr --show | string match -- "*ㅅㅊ*")
        if string match -q "*--set-cursor*" -- $def
            print_test_result "auto-register: --set-cursor is preserved" 0
        else
            print_test_result "auto-register: --set-cursor is preserved" 1
        end
        if string match -q "*--position anywhere*" -- $def
            print_test_result "auto-register: --position anywhere is preserved with --set-cursor" 0
        else
            print_test_result "auto-register: --position anywhere is preserved with --set-cursor" 1
        end
    else
        print_test_result "auto-register: --set-cursor is preserved" 1
    end

    # Cleanup
    abbr -e -- testcmd_SC
    abbr -e -- "ㅅㅊ"
end

function test_vltl_path_env_var
    echo ""
    echo "Testing VLTL_PATH environment variable support..."

    # Test 1: VLTL_PATH is used by __vltl_convert_and_expand
    set -l vltl_bin (which vltl)
    set -gx VLTL_PATH $vltl_bin
    vltl init | source

    # Verify function definitions still work with VLTL_PATH
    if type -q __vltl_convert_and_expand
        print_test_result "VLTL_PATH: functions defined with custom path" 0
    else
        print_test_result "VLTL_PATH: functions defined with custom path" 1
    end

    # Test 2: vltl convert still works (binary command, independent of VLTL_PATH)
    set -l converted (vltl convert "ㅣ")
    if test "$converted" = l
        print_test_result "VLTL_PATH: vltl convert works correctly" 0
    else
        print_test_result "VLTL_PATH: vltl convert works correctly" 1
    end

    # Cleanup
    set -e VLTL_PATH
end

function test_switch_to_english_command
    echo ""
    echo "Testing switch-to-english command availability..."

    # Check if we're on macOS
    if test (uname -s) = Darwin
        echo "Running on macOS - testing switch-to-english command"

        # Test 1: Command should exist on macOS
        if vltl help | grep -q switch-to-english
            print_test_result "switch-to-english command exists on macOS" 0
        else
            print_test_result "switch-to-english command exists on macOS" 1
        end

        # Test 2: Command should execute without error (even if IME doesn't change)
        if vltl switch-to-english 2>&1 | grep -qv "error: unrecognized subcommand"
            print_test_result "switch-to-english command executes on macOS" 0
        else
            print_test_result "switch-to-english command executes on macOS" 1
        end
    else
        echo "Running on Linux - verifying switch-to-english is not available"

        # Test: Command should NOT exist on Linux
        if not vltl help | grep -q switch-to-english
            print_test_result "switch-to-english command not available on Linux" 0
        else
            print_test_result "switch-to-english command not available on Linux" 1
        end

        # Test: Calling it should fail gracefully (exit code 2)
        vltl switch-to-english 2>/dev/null
        if test $status -eq 2
            print_test_result "switch-to-english fails gracefully on Linux" 0
        else
            print_test_result "switch-to-english fails gracefully on Linux" 1
        end
    end
end

function test_extract_programs_command
    echo ""
    echo "Testing vltl extract-programs command..."

    # Test: Simple command extraction
    set -l result (vltl extract-programs -- "ls -la")
    if test "$result" = ls
        print_test_result "extract-programs: simple command" 0
    else
        print_test_result "extract-programs: simple command (got: $result)" 1
    end

    # Test: Korean command extraction
    set -l result (vltl extract-programs -- "ㅣㄴ -la")
    if test "$result" = "ㅣㄴ"
        print_test_result "extract-programs: Korean command" 0
    else
        print_test_result "extract-programs: Korean command (got: $result)" 1
    end
end

function test_integration_convert_flow
    echo ""
    echo "Testing integration conversion flow..."

    # Source the init script
    vltl init | source

    # Test the full conversion pipeline:
    # 1. Korean input detected
    # 2. Converted to English
    # 3. Can be used as abbr trigger

    # Setup: register an abbr for the converted result
    set -l korean_input "햣"
    set -l expected_english "git"
    set -l converted (vltl convert "$korean_input")

    if test "$converted" = "$expected_english"
        print_test_result "integration: 햣 correctly converts to git" 0
    else
        print_test_result "integration: 햣 correctly converts to git (got: $converted)" 1
    end

    # Register abbr for the English trigger
    abbr -a -- "$expected_english" "git status"

    # The auto-register should create a Korean trigger abbr
    __vltl_auto_register_abbr "$korean_input" "$expected_english"

    if abbr -q -- "$korean_input"
        print_test_result "integration: Korean abbr auto-registered for converted trigger" 0
    else
        print_test_result "integration: Korean abbr auto-registered for converted trigger" 1
    end

    # Cleanup
    abbr -e -- "$expected_english"
    abbr -e -- "$korean_input"
end

# Run all tests
echo "========================================"
echo "Running vltl E2E Tests"
echo "Fish Abbr Integration Tests"
echo "========================================"

test_function_definitions
test_convert_command
test_has_korean_command
test_auto_register_abbr
test_auto_register_preserves_options
test_auto_register_no_duplicate
test_auto_register_with_set_cursor
test_vltl_path_env_var
test_switch_to_english_command
test_extract_programs_command
test_integration_convert_flow

# Print summary
echo ""
echo "========================================"
echo "Test Summary"
echo "========================================"
echo "Passed: $test_passed"
echo "Failed: $test_failed"
echo "========================================"

if test $test_failed -gt 0
    exit 1
else
    echo "All tests passed!"
    exit 0
end
