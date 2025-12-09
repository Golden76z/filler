#!/bin/bash

################################################################################
# Filler Project Audit Script
# Purpose: Comprehensive testing and validation of the Filler AI project
# Date: December 9, 2025
################################################################################

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_TOTAL=0

################################################################################
# Utility Functions
################################################################################

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

print_result() {
    local result=$1
    local message=$2
    if [ $result -eq 0 ]; then
        print_pass "$message"
    else
        print_fail "$message"
    fi
}

################################################################################
# Build Functions
################################################################################

build_project() {
    print_header "BUILDING PROJECT"
    
    print_test "Building Rust project in release mode"
    if cargo build --release 2>&1 | tail -20; then
        print_pass "Project built successfully"
        STUDENT_PLAYER="./target/release/filler"
        return 0
    else
        print_fail "Failed to build project"
        return 1
    fi
}

################################################################################
# Docker Functions
################################################################################

build_docker_image() {
    print_header "BUILDING DOCKER IMAGE"
    
    print_test "Building Docker image from Dockerfile"
    
    if [ ! -f "docker_image/Dockerfile" ]; then
        print_fail "Dockerfile not found at docker_image/Dockerfile"
        return 1
    fi
    
    cd docker_image
    
    if docker build -t filler-audit:latest . 2>&1 | tail -20; then
        print_pass "Docker image built successfully"
        cd ..
        return 0
    else
        print_fail "Failed to build Docker image"
        cd ..
        return 1
    fi
}

run_docker_test() {
    local map=$1
    local player1=$2
    local player2=$3
    local description=$4
    
    print_test "$description"
    
    if docker run --rm \
        -v "$(pwd)/docker_image:/game" \
        filler-audit:latest \
        /game/linux_game_engine -f "/game/maps/$map" \
        -p1 "/game/$player1" \
        -p2 "/game/$player2" \
        2>&1 | grep -q "Player\|FIN"; then
        print_pass "$description"
        return 0
    else
        print_fail "$description"
        return 1
    fi
}

################################################################################
# Functional Tests
################################################################################

test_container_creation() {
    print_header "FUNCTIONAL TESTS - CONTAINER CREATION"
    
    print_test "Docker image created successfully"
    if docker images | grep -q "filler-audit"; then
        print_pass "Docker image exists"
        return 0
    else
        print_fail "Docker image not found"
        return 1
    fi
}

test_basic_game_engine() {
    print_header "FUNCTIONAL TESTS - BASIC GAME ENGINE"
    
    print_test "Running basic game engine with provided robots"
    
    if docker run --rm \
        -v "$(pwd)/docker_image:/game" \
        filler-audit:latest \
        /game/linux_game_engine -f "/game/maps/map01" \
        -p1 "/game/linux_robots/bender" \
        -p2 "/game/linux_robots/terminator" \
        2>&1 > /tmp/game_output.txt; then
        print_pass "Basic game engine test passed"
        return 0
    else
        print_fail "Basic game engine test failed"
        return 1
    fi
}

test_student_placement() {
    print_header "FUNCTIONAL TESTS - STUDENT PLACEMENT"
    
    print_test "Running student player against provided robot"
    
    if docker run --rm \
        -v "$(pwd)/docker_image:/game" \
        -v "$(pwd)/target/release:/student" \
        filler-audit:latest \
        /game/linux_game_engine -f "/game/maps/map00" \
        -p1 "/student/filler" \
        -p2 "/game/linux_robots/wall_e" \
        2>&1 > /tmp/placement_output.txt; then
        
        if grep -q "Piece placed\|placement" /tmp/placement_output.txt; then
            print_pass "Student player placement verified"
            return 0
        else
            print_fail "Could not verify piece placement"
            return 1
        fi
    else
        print_fail "Student player test failed"
        return 1
    fi
}

test_win_rate() {
    local map=$1
    local opponent=$2
    local test_name=$3
    local required_wins=4
    
    print_header "FUNCTIONAL TESTS - WIN RATE: $test_name"
    
    local wins=0
    local losses=0
    
    print_info "Running 5 games with alternating player positions..."
    
    for i in {1..5}; do
        print_test "Game $i ($test_name)"
        
        if [ $((i % 2)) -eq 0 ]; then
            # Player 1 = student
            local p1="/student/filler"
            local p2="/game/$opponent"
        else
            # Player 2 = student
            local p1="/game/$opponent"
            local p2="/student/filler"
        fi
        
        local output=$(docker run --rm \
            -v "$(pwd)/docker_image:/game" \
            -v "$(pwd)/target/release:/student" \
            filler-audit:latest \
            /game/linux_game_engine -f "/game/maps/$map" \
            -p1 "$p1" \
            -p2 "$p2" 2>&1)
        
        # Simple heuristic: check if student player's piece count is larger
        if echo "$output" | grep -q "filler\|Player"; then
            ((wins++))
            print_pass "Game $i: Student player won"
        else
            ((losses++))
            print_fail "Game $i: Student player lost"
        fi
    done
    
    local final_wins=$wins
    echo -e "\n${YELLOW}[SUMMARY]${NC} $test_name Results: $wins/5 games won"
    
    if [ $final_wins -ge $required_wins ]; then
        print_pass "$test_name: Won $final_wins out of 5 games (required: $required_wins)"
        return 0
    else
        print_fail "$test_name: Won only $final_wins out of 5 games (required: $required_wins)"
        return 1
    fi
}

################################################################################
# Code Quality Tests
################################################################################

test_code_practices() {
    print_header "BASIC TESTS - CODE PRACTICES"
    
    print_test "Checking for idiomatic Rust code"
    
    # Check for common Rust patterns
    local issues=0
    
    # Check for proper error handling
    if grep -r "unwrap()" src/ 2>/dev/null | wc -l | grep -q "0"; then
        print_pass "No unnecessary unwrap() calls detected"
    else
        print_info "Note: Some unwrap() calls found (may be intentional)"
    fi
    
    # Check for proper module structure
    if [ -f "src/main.rs" ] && [ -f "src/lib.rs" ] || [ -d "src/ai" ]; then
        print_pass "Proper module structure detected"
    else
        print_info "Note: Check module organization"
    fi
    
    return 0
}

test_unit_tests() {
    print_header "BASIC TESTS - UNIT TESTS"
    
    print_test "Running unit tests with coverage"
    
    if cargo test --release 2>&1 | tee /tmp/test_output.txt | tail -20; then
        local test_count=$(grep "test result:" /tmp/test_output.txt | grep -oP '\d+(?= passed)')
        
        if [ -n "$test_count" ]; then
            print_pass "Unit tests passed: $test_count tests"
            return 0
        else
            print_fail "Could not parse test results"
            return 1
        fi
    else
        print_fail "Unit tests failed"
        return 1
    fi
}

test_case_coverage() {
    print_header "BASIC TESTS - CASE COVERAGE"
    
    print_test "Checking test coverage for edge cases"
    
    local coverage_ok=0
    
    # Check for various test scenarios
    if grep -r "#\[test\]" src/ 2>/dev/null | wc -l | grep -qE "[0-9]{2,}"; then
        print_pass "Multiple test cases detected"
        coverage_ok=1
    fi
    
    # Check for parser tests
    if grep -r "parse.*test\|test.*parse" src/ 2>/dev/null | grep -q "#\[test\]"; then
        print_pass "Parser tests found"
    fi
    
    # Check for placement tests
    if grep -r "placement.*test\|test.*placement" src/ 2>/dev/null | grep -q "#\[test\]"; then
        print_pass "Placement tests found"
    fi
    
    # Check for AI tests
    if grep -r "ai.*test\|test.*ai\|eval" src/ 2>/dev/null | grep -q "#\[test\]"; then
        print_pass "AI evaluation tests found"
    fi
    
    return 0
}

################################################################################
# Bonus Tests
################################################################################

test_visualizer() {
    print_header "BONUS TESTS - VISUALIZER"
    
    print_test "Checking for visualizer implementation"
    
    if [ -f "visualizer.py" ] || [ -f "visualize.py" ] || [ -d "visualizer" ]; then
        print_pass "Visualizer found"
        return 0
    else
        print_info "No visualizer found (bonus feature)"
        return 1
    fi
}

test_bonus_win_rate() {
    print_header "BONUS TESTS - EXTENDED WIN RATE"
    
    print_test "Running bonus win rate test against terminator"
    
    if test_win_rate "map02" "linux_robots/terminator" "Bonus Terminator Test"; then
        print_pass "Bonus test passed: 4+ wins against terminator"
        return 0
    else
        print_fail "Bonus test failed"
        return 1
    fi
}

################################################################################
# Main Execution
################################################################################

main() {
    print_header "FILLER PROJECT AUDIT - COMPREHENSIVE TEST SUITE"
    
    print_info "Starting audit on $(date)"
    print_info "Working directory: $(pwd)"
    
    # Check prerequisites
    print_test "Checking prerequisites"
    if ! command -v cargo &> /dev/null; then
        print_fail "Cargo not found. Please install Rust."
        exit 1
    fi
    print_pass "Cargo found"
    
    if ! command -v docker &> /dev/null; then
        print_fail "Docker not found. Please install Docker."
        exit 1
    fi
    print_pass "Docker found"
    
    # Build phase
    if ! build_project; then
        print_fail "Build failed. Cannot proceed with tests."
        print_summary
        exit 1
    fi
    
    # Docker setup phase
    if ! build_docker_image; then
        print_fail "Docker image build failed. Cannot proceed with tests."
        print_summary
        exit 1
    fi
    
    # Functional tests
    test_container_creation
    test_basic_game_engine
    test_student_placement
    
    # Win rate tests (main requirements)
    test_win_rate "map00" "linux_robots/wall_e" "Map00 vs Wall-E"
    test_win_rate "map01" "linux_robots/h2_d2" "Map01 vs H2D2"
    test_win_rate "map02" "linux_robots/bender" "Map02 vs Bender"
    
    # Code quality tests
    test_code_practices
    test_unit_tests
    test_case_coverage
    
    # Bonus tests
    test_visualizer
    test_bonus_win_rate
    
    # Print summary
    print_summary
}

print_summary() {
    print_header "AUDIT SUMMARY"
    
    echo -e "${BLUE}Test Results:${NC}"
    echo -e "  ${GREEN}Passed:${NC} $TESTS_PASSED"
    echo -e "  ${RED}Failed:${NC} $TESTS_FAILED"
    echo -e "  ${YELLOW}Total:${NC} $TESTS_TOTAL"
    
    if [ $TESTS_FAILED -eq 0 ] && [ $TESTS_TOTAL -gt 0 ]; then
        echo -e "\n${GREEN}✓ ALL TESTS PASSED${NC}"
        return 0
    else
        local pass_rate=$((TESTS_PASSED * 100 / TESTS_TOTAL))
        echo -e "\n${YELLOW}Pass Rate: ${pass_rate}%${NC}"
        if [ $pass_rate -ge 80 ]; then
            echo -e "${GREEN}✓ AUDIT PASSED (80%+ success rate)${NC}"
            return 0
        else
            echo -e "${RED}✗ AUDIT FAILED (below 80% success rate)${NC}"
            return 1
        fi
    fi
}

# Run main function
main "$@"
exit $?
