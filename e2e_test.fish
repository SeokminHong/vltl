#!/usr/bin/env fish

# E2E test script for vltl
# This script tests the vltl command-line tool and fish integration

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

function test_convert_command
    echo ""
    echo "Testing convert command..."
    
    # Test 1: Convert ㅔㅞㅡ to pnpm
    set -l result (vltl convert "ㅔㅞㅡ")
    if test "$result" = "pnpm"
        print_test_result "convert ㅔㅞㅡ -> pnpm" 0
    else
        echo "  Expected: pnpm, Got: $result"
        print_test_result "convert ㅔㅞㅡ -> pnpm" 1
    end
    
    # Test 2: Convert ㅍㅣ to vl
    set -l result (vltl convert "ㅍㅣ")
    if test "$result" = "vl"
        print_test_result "convert ㅍㅣ -> vl" 0
    else
        echo "  Expected: vl, Got: $result"
        print_test_result "convert ㅍㅣ -> vl" 1
    end
    
    # Test 3: Convert ㅛㅁ구 to yarn
    set -l result (vltl convert "ㅛㅁ구")
    if test "$result" = "yarn"
        print_test_result "convert ㅛㅁ구 -> yarn" 0
    else
        echo "  Expected: yarn, Got: $result"
        print_test_result "convert ㅛㅁ구 -> yarn" 1
    end
    
    # Test 4: Convert ㅎㄱ데 to grep
    set -l result (vltl convert "ㅎㄱ데")
    if test "$result" = "grep"
        print_test_result "convert ㅎㄱ데 -> grep" 0
    else
        echo "  Expected: grep, Got: $result"
        print_test_result "convert ㅎㄱ데 -> grep" 1
    end
end

function test_has_korean_command
    echo ""
    echo "Testing has-korean command..."
    
    # Test 1: Has Korean - should return 0
    vltl has-korean "ㅔㅞㅡ"
    if test $status -eq 0
        print_test_result "has-korean ㅔㅞㅡ (should have Korean)" 0
    else
        print_test_result "has-korean ㅔㅞㅡ (should have Korean)" 1
    end
    
    # Test 2: Has Korean - completed form
    vltl has-korean "피시"
    if test $status -eq 0
        print_test_result "has-korean 피시 (should have Korean)" 0
    else
        print_test_result "has-korean 피시 (should have Korean)" 1
    end
    
    # Test 3: No Korean - should return 1
    vltl has-korean "npm"
    if test $status -eq 1
        print_test_result "has-korean npm (should NOT have Korean)" 0
    else
        print_test_result "has-korean npm (should NOT have Korean)" 1
    end
    
    # Test 4: No Korean - empty string
    vltl has-korean ""
    if test $status -eq 1
        print_test_result "has-korean empty string (should NOT have Korean)" 0
    else
        print_test_result "has-korean empty string (should NOT have Korean)" 1
    end
end

function test_init_command
    echo ""
    echo "Testing init command..."
    
    # Test: init command should output fish script
    set -l result (vltl init)
    
    if string match -q "*function __vltl*" -- $result
        print_test_result "init outputs fish script with __vltl function" 0
    else
        print_test_result "init outputs fish script with __vltl function" 1
    end
    
    if string match -q "*function __vltl_check*" -- $result
        print_test_result "init outputs fish script with __vltl_check function" 0
    else
        print_test_result "init outputs fish script with __vltl_check function" 1
    end
end

function test_fish_integration
    echo ""
    echo "Testing fish shell integration..."
    
    # Source the init script
    vltl init | source
    
    # Test __vltl_check function exists
    if type -q __vltl_check
        print_test_result "__vltl_check function is defined" 0
    else
        print_test_result "__vltl_check function is defined" 1
    end
    
    # Test __vltl function exists
    if type -q __vltl
        print_test_result "__vltl function is defined" 0
    else
        print_test_result "__vltl function is defined" 1
    end
    
    # Test __vltl_check with existing command
    if __vltl_check echo
        print_test_result "__vltl_check returns true for existing command (echo)" 0
    else
        print_test_result "__vltl_check returns true for existing command (echo)" 1
    end
    
    # Test __vltl_check with non-existing command
    if not __vltl_check nonexistentcommand123456
        print_test_result "__vltl_check returns false for non-existing command" 0
    else
        print_test_result "__vltl_check returns false for non-existing command" 1
    end
end

function test_version_command
    echo ""
    echo "Testing version command..."
    
    set -l result (vltl --version)
    
    if string match -q "vltl *" -- $result
        print_test_result "vltl --version outputs version" 0
    else
        echo "  Got: $result"
        print_test_result "vltl --version outputs version" 1
    end
end

# Run all tests
echo "========================================"
echo "Running vltl E2E Tests"
echo "========================================"

test_version_command
test_convert_command
test_has_korean_command
test_init_command
test_fish_integration

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
