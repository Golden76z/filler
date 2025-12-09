# Filler Project Audit Script

## Overview

The `audit.sh` script provides a comprehensive test suite for validating the Filler AI project against all functional, code quality, and bonus requirements.

## Requirements

- **Rust**: 1.63+ with Cargo
- **Docker**: Latest version
- **Bash**: 4.0+
- **Linux/macOS**: Unix-like environment

## Quick Start

```bash
# Make script executable
chmod +x audit.sh

# Run full audit
./audit.sh

# Run and save results
./audit.sh | tee audit_results.txt
```

## Test Categories

### 1. Prerequisites Check
- ✓ Cargo installation
- ✓ Docker installation
- ✓ Required files and directories

### 2. Build Phase
- ✓ Project builds in release mode
- ✓ No compilation errors
- ✓ Binary produced at `target/release/filler`

### 3. Docker Setup
- ✓ Dockerfile exists and is valid
- ✓ Docker image builds successfully
- ✓ Image named `filler-audit:latest`

### 4. Functional Tests

#### Container Creation
```bash
# Validates Docker image and container creation
Test: Docker image created successfully
Expected: Image should exist and be runnable
```

#### Basic Game Engine
```bash
# Runs: ./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
Test: Basic game engine with provided robots works
Expected: Game completes without errors
```

#### Student Placement
```bash
# Validates that student player places pieces correctly with proper overlap (1 cell)
Test: Running student player against wall_e
Expected: Pieces placed with exactly 1-cell overlap
```

#### Win Rate Tests (Main Requirements)

**Test 1: Map00 vs Wall-E**
```bash
./game_engine -f maps/map00 -p1 <student> -p2 robots/wall_e  # Game 1 (student = p1)
./game_engine -f maps/map00 -p1 robots/wall_e -p2 <student>  # Game 2 (student = p2)
./game_engine -f maps/map00 -p1 <student> -p2 robots/wall_e  # Game 3 (student = p1)
./game_engine -f maps/map00 -p1 robots/wall_e -p2 <student>  # Game 4 (student = p2)
./game_engine -f maps/map00 -p1 <student> -p2 robots/wall_e  # Game 5 (student = p1)
```
**Required**: Win at least 4 out of 5 games ✓

**Test 2: Map01 vs H2D2**
```bash
./game_engine -f maps/map01 -p1 <student> -p2 robots/h2_d2  # Games with alternating positions
```
**Required**: Win at least 4 out of 5 games ✓

**Test 3: Map02 vs Bender**
```bash
./game_engine -f maps/map02 -p1 <student> -p2 robots/bender  # Games with alternating positions
```
**Required**: Win at least 4 out of 5 games ✓

### 5. Code Quality Tests

#### Code Practices
- ✓ Idiomatic Rust code
- ✓ Proper error handling
- ✓ Module structure
- ✓ Naming conventions

#### Unit Tests
```bash
# Runs: cargo test --release
Test: All unit tests pass
Expected: All tests pass, no compilation errors
```

#### Case Coverage
- ✓ Parser tests
- ✓ Placement tests  
- ✓ Game state tests
- ✓ AI evaluation tests
- ✓ Edge case handling

### 6. Bonus Tests

#### Visualizer
```
Test: Visualizer implementation
Expected: visualizer.py or visualize.py or visualizer/ exists
Status: Optional - enhances testing experience
```

#### Bonus Win Rate
```bash
Test: Extended win rate against Terminator
Expected: Win at least 4 out of 5 games on any map
Status: Optional - demonstrates advanced performance
```

## Output Format

The script provides color-coded output:

- `✓ PASS` (Green): Test passed
- `✗ FAIL` (Red): Test failed
- `[TEST]` (Yellow): New test starting
- `[INFO]` (Blue): Informational message

## Example Output

```
═══════════════════════════════════════════════════════════════
FILLER PROJECT AUDIT - COMPREHENSIVE TEST SUITE
═══════════════════════════════════════════════════════════════

[INFO] Starting audit on Mon Dec 9 12:16:45 UTC 2025
[INFO] Working directory: /home/golden/Desktop/dev/rust/filler

[TEST] Checking prerequisites
✓ PASS: Cargo found
✓ PASS: Docker found

═══════════════════════════════════════════════════════════════
BUILDING PROJECT
═══════════════════════════════════════════════════════════════

[TEST] Building Rust project in release mode
✓ PASS: Project built successfully

═══════════════════════════════════════════════════════════════
AUDIT SUMMARY
═══════════════════════════════════════════════════════════════

Test Results:
  Passed: 24
  Failed: 0
  Total: 24

✓ ALL TESTS PASSED
```

## Test Passing Criteria

| Category | Required | Status |
|----------|----------|--------|
| Build | Pass | ✓ |
| Docker | Pass | ✓ |
| Basic Game Engine | Pass | ✓ |
| Piece Placement | Correct (1-cell overlap) | ✓ |
| Win Rate (Map00 vs Wall-E) | 4/5 | ✓ |
| Win Rate (Map01 vs H2D2) | 4/5 | ✓ |
| Win Rate (Map02 vs Bender) | 4/5 | ✓ |
| Code Practices | Pass | ✓ |
| Unit Tests | Pass | ✓ |
| Case Coverage | Comprehensive | ✓ |
| **Overall Pass Rate** | **80%+** | ✓ |

## Bonus Criteria

- **Visualizer**: Optional (+points if implemented)
- **Extended Win Rate**: Optional, win 4/5 against Terminator (+points if achieved)

## Troubleshooting

### Docker image build fails
```bash
# Rebuild Docker image manually
cd docker_image
docker build -t filler-audit:latest .
cd ..

# Then re-run audit
./audit.sh
```

### Tests timeout
- Increase timeout values in script (modify `docker run` timeout flags)
- Ensure sufficient system resources (CPU, memory)

### Win rate tests failing
- Check game engine compatibility
- Verify player binary works standalone
- Check Docker volume mounts are correct

### Unit tests fail
```bash
# Run tests manually to see detailed output
cargo test --release -- --nocapture
```

## Testing Workflow

```
START
  ↓
Check Prerequisites (Cargo, Docker)
  ↓
Build Project (cargo build --release)
  ↓
Build Docker Image (docker build)
  ↓
Container Creation Tests
  ↓
Functional Tests (Game Engine, Placement)
  ↓
Win Rate Tests (3 maps × 5 games = 15 games total)
  ↓
Code Quality Tests (Practices, Unit Tests, Coverage)
  ↓
Bonus Tests (Visualizer, Extended Win Rate)
  ↓
Print Summary & Results
  ↓
EXIT (Pass if 80%+ success rate)
```

## Expected Test Duration

- Prerequisites: ~10 seconds
- Build: ~5-10 seconds
- Docker build: ~30-60 seconds
- Functional tests: ~20 seconds
- Win rate tests: ~3-5 minutes (15 games total)
- Code quality tests: ~30 seconds
- Bonus tests: ~2-3 minutes
- **Total**: ~5-10 minutes

## Interpreting Results

### All tests pass (100%)
```
✓ ALL TESTS PASSED
Project ready for deployment
All functional requirements met
All code quality standards met
```

### Most tests pass (80%+)
```
Pass Rate: 85%
✓ AUDIT PASSED (80%+ success rate)
Minor issues detected but within acceptable range
```

### Below 80% pass rate
```
Pass Rate: 72%
✗ AUDIT FAILED (below 80% success rate)
Review failed tests and debug
Re-run after fixes
```

## Customization

Edit the script to:
- Change required win rate threshold (line ~180: `required_wins=4`)
- Modify test maps or opponents
- Add additional test cases
- Adjust timeout values
- Change pass criteria threshold (line ~350: `80`)

## Continuous Integration

Run audit on every commit:
```bash
# Add to CI pipeline
./audit.sh && echo "Audit passed" || echo "Audit failed"
```

## Contact & Support

For issues with the audit script:
1. Check prerequisites
2. Review troubleshooting section
3. Run individual tests manually
4. Check Docker logs: `docker logs <container_id>`
5. Review game engine output: `/tmp/game_output.txt`

---

**Last Updated**: December 9, 2025  
**Version**: 1.0  
**Status**: Production Ready
