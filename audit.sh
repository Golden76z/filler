#!/bin/bash

################################################################################
# Filler Project Audit Script - Simple Version
# Purpose: Test the Filler AI project
# Date: December 9, 2025
################################################################################

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Counters
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_TOTAL=0

print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}\n"
}

print_test() {
    echo -e "${YELLOW}[TEST]${NC} $1"
    ((TESTS_TOTAL++))
}

print_pass() {
    echo -e "${GREEN}✓ PASS${NC}: $1"
    ((TESTS_PASSED++))
}

print_fail() {
    echo -e "${RED}✗ FAIL${NC}: $1"
    ((TESTS_FAILED++))
}

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

################################################################################
# Build & Test Functions
################################################################################

test_build() {
    print_header "BUILD & COMPILATION TESTS"
    
    print_test "Building project in release mode"
    if cargo build --release 2>&1 | grep -q "Finished"; then
        print_pass "Project built successfully"
    else
        print_fail "Build failed"
        return 1
    fi
}

test_unit_tests() {
    print_header "UNIT TESTS"
    
    print_test "Running all unit tests"
    if cargo test --release 2>&1 | grep -q "test result: ok"; then
        local count=$(cargo test --release 2>&1 | grep "test result" | grep -oP '\d+(?= passed)')
        print_pass "All unit tests passed ($count tests)"
    else
        print_fail "Some unit tests failed"
        return 1
    fi
}

test_code_quality() {
    print_header "CODE QUALITY CHECKS"
    
    print_test "Checking project structure"
    if [ -f "src/main.rs" ] && [ -d "src/ai" ]; then
        print_pass "Project structure is valid"
    else
        print_fail "Invalid project structure"
        return 1
    fi
    
    print_test "Checking documentation"
    if [ -f "README.md" ] && [ -f "PROGRESS.md" ]; then
        print_pass "Documentation files found"
    else
        print_fail "Missing documentation"
        return 1
    fi
}

test_binary() {
    print_header "BINARY VALIDATION"
    
    print_test "Checking release binary"
    if [ -f "target/release/filler" ]; then
        print_pass "Release binary exists"
        
        print_test "Checking binary is executable"
        if [ -x "target/release/filler" ]; then
            print_pass "Binary is executable"
        else
            print_fail "Binary is not executable"
            return 1
        fi
    else
        print_fail "Release binary not found"
        return 1
    fi
}

test_docker_setup() {
    print_header "DOCKER SETUP"
    
    print_test "Checking Docker availability"
    if command -v docker &> /dev/null; then
        print_pass "Docker is installed"
        
        print_test "Checking Dockerfile"
        if [ -f "docker_image/Dockerfile" ]; then
            print_pass "Dockerfile found"
        else
            print_fail "Dockerfile not found"
            return 1
        fi
    else
        print_info "Docker not available (optional)"
        return 0
    fi
}

print_summary() {
    print_header "AUDIT SUMMARY"
    
    echo -e "${BLUE}Test Results:${NC}"
    echo -e "  ${GREEN}Passed:${NC} $TESTS_PASSED"
    echo -e "  ${RED}Failed:${NC} $TESTS_FAILED"
    echo -e "  ${YELLOW}Total:${NC} $TESTS_TOTAL"
    echo ""
    
    if [ $TESTS_TOTAL -eq 0 ]; then
        echo -e "${YELLOW}No tests ran${NC}"
        return 1
    fi
    
    local pass_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
    echo -e "${BLUE}Pass Rate: ${pass_rate}%${NC}"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "\n${GREEN}✓ ALL TESTS PASSED${NC}"
        return 0
    elif [ $pass_rate -ge 80 ]; then
        echo -e "\n${GREEN}✓ AUDIT PASSED (80%+ success rate)${NC}"
        return 0
    else
        echo -e "\n${RED}✗ AUDIT FAILED (below 80% success rate)${NC}"
        return 1
    fi
}

main() {
    print_header "FILLER PROJECT AUDIT"
    
    print_info "Starting audit on $(date)"
    print_info "Working directory: $(pwd)"
    
    # Run all tests
    test_build || true
    test_unit_tests || true
    test_code_quality || true
    test_binary || true
    test_docker_setup || true
    
    # Print summary
    print_summary
}

main "$@"
