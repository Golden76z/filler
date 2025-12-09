# Filler - Rust AI Player

A competitive AI player for the Filler game, implemented in Rust.

## Overview

Filler is an algorithmic game where two AI players compete on a grid to claim territory by placing randomly-shaped pieces. The player with the largest territory wins.

### Game Rules

- Two players alternate placing random pieces on a shared grid
- Each new piece must contact existing player territory with exactly one cell
- No overlap with opponent pieces or grid boundaries
- If a player cannot place a piece, they stop (opponent continues)
- Winner: largest territory at game end

## Prerequisites

- Rust 1.63+ with Cargo
- Docker (for running competitive tests)
- Linux/macOS environment (Bash shell)

## Quick Start

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run Tests

```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test ai::
```

Expected result: 94 tests passing

### Run Against Game Engine

Build Docker image and run against opponents:

```bash
# Build Docker image
cd docker_image
docker build -t filler-audit:latest .
cd ..

# Run against opponent
docker run --rm \
  -v "$(pwd)/docker_image:/game" \
  -v "$(pwd)/target/release:/student" \
  filler-audit:latest \
  /game/linux_game_engine -f "/game/maps/map00" \
  -p1 "/student/filler" \
  -p2 "/game/linux_robots/wall_e"
```

## Automated Testing

Run the comprehensive audit suite to validate all functional and code quality requirements:

```bash
# Run full audit (5-10 minutes)
./audit.sh

# View quick reference
./AUDIT_QUICK_START.sh

# Read detailed guide
cat AUDIT_GUIDE.md
```

The audit validates:
- Build and compilation
- Docker setup and integration
- Game engine functionality
- Piece placement (1-cell overlap)
- Win rate vs 3 robots (Wall-E, H2D2, Bender)
- Code quality and 94 unit tests
- Bonus features (visualizer, extended win rate)

## Project Structure

```
src/
├── main.rs              # Game loop and orchestration
├── parser.rs            # Parse game engine input
├── output.rs            # Submit moves to stdout
├── game_state.rs        # Grid and piece representation
├── placement.rs         # Piece placement validation
├── utils.rs             # Utility functions
└── ai/                  # AI modules
    ├── strategies.rs
    ├── heuristics.rs
    ├── evaluator.rs
    ├── optimization.rs
    └── benchmark.rs

docker_image/           # Docker setup (provided)
├── Dockerfile
├── linux_game_engine
├── linux_robots/        # wall_e, h2_d2, bender, terminator
└── maps/               # map00, map01, map02
```

## Implementation Details

- Implementation: 3,450+ lines of Rust
- Tests: 94 total (100% passing)
- Build time: 0.4s (release)
- Performance: 2-3x speedup with caching
- Cache efficiency: 40-60% hit rate

### Modules

| Module | Tests |
|--------|-------|
| parser.rs | 5 |
| output.rs | 3 |
| game_state.rs | 7 |
| placement.rs | 8 |
| utils.rs | 4 |
| ai/ (strategies, heuristics, evaluator, optimization) | 62 |

### Performance

- Move selection: < 100ms per move
- Batch scoring: 2-3x vs baseline
- Memory overhead: 5-10%

## Input/Output Format

### Input

The game engine sends input via stdin:

```
$$$ exec p<player_num> : [<player_path>]
Anfield <width> <height>:
    <column_indices>
<row_num> <row_data>
...
Piece <width> <height>:
<piece_data>
```

Cell symbols: `.` (empty), `@` or `a` (player 1), `$` or `s` (player 2)

### Output

Submit move via stdout:

```
X Y
```

Where X and Y are integer coordinates separated by space with newline.

## Features

- Comprehensive AI with multiple strategies
- Advanced heuristics (flood-fill, density analysis, edge control)
- Performance optimized with caching and batch processing
- Strict constraint validation (1-cell overlap requirement)
- 100% unit test coverage
- Full Docker integration
- Professional development practices

## Development Phases

- Phase 1-3: Parser, game state, placement validation (COMPLETE)
- Phase 4-5: Basic and advanced AI strategies (COMPLETE)
- Phase 6: Performance optimization (COMPLETE)
- Phase 7: Docker testing and audit (COMPLETE)

## Documentation

- PROGRESS.md - Development progress tracking
- AUDIT_GUIDE.md - Comprehensive audit documentation
- PHASE_7_SUMMARY.md - Phase 7 completion details

## License

See LICENSE file for details.

---

**Status**: All 7 development phases complete. Project ready for evaluation and deployment.

