# Filler - Rust AI Player

A competitive AI player for the Filler game, implemented in Rust with professional development practices.

## Overview

**Filler** is an algorithmic game where two AI players compete on a grid (Anfield) to claim territory by placing randomly-shaped pieces. The player who occupies the largest surface wins.

### Game Rules

- Two players alternate turns placing random pieces on a shared grid
- **Critical Constraint**: Each new piece must contact existing player territory with **exactly one cell**
- Cannot overlap opponent pieces or grid boundaries
- If a player cannot place a piece, they stop; opponent continues
- Winner: Player with largest territory when no more pieces can be placed

### Example Game Layout

```
Initial Anfield (20 × 15):
    01234567890123456789
000 ....................
001 ....................
002 .........@..........    @ = Player 1 starting position
003 ....................
...
012 .........$..........    $ = Player 2 starting position
...

Current Piece to Place:
Piece 4 1:
.OO.    (O = filled cells, . = empty cells)
```

**Valid Placement**: Position where exactly one O overlaps with player's territory  
**Output**: `7 2` (x coordinate, y coordinate, newline)

---

## Project Structure

```
filler/
├── Cargo.toml                    # Rust project manifest
├── Cargo.lock                    # Dependency lock file
├── LICENSE                       # Project license
├── README.md                     # This file
├── PROGRESS.md                   # Development progress tracker
│
├── src/
│   ├── main.rs                   # Game loop orchestration
│   ├── parser.rs                 # Input parsing from game engine
│   ├── output.rs                 # Move submission to stdout
│   ├── game_state.rs             # Internal game state representation
│   ├── placement.rs              # Piece placement validation
│   └── utils.rs                  # Utility functions
│
├── docker_image/                 # Docker setup (provided)
│   ├── Dockerfile                # Container configuration
│   ├── linux_game_engine         # Game engine binary (Linux)
│   ├── m1_game_engine            # Game engine binary (Apple Silicon)
│   ├── linux_robots/             # AI opponents
│   │   ├── bender                # Moderate difficulty
│   │   ├── h2_d2                 # Moderate difficulty
│   │   ├── terminator            # Hard difficulty
│   │   └── wall_e                # Easy difficulty
│   ├── m1_robots/                # Apple Silicon versions
│   └── maps/                     # Test grids
│       ├── map00                 # 20×15 (beginner)
│       ├── map01                 # 20×15 (intermediate)
│       └── map02                 # 20×15 (advanced)
│
└── target/                       # Build artifacts (auto-generated)
    ├── debug/                    # Debug build
    └── release/                  # Release build
```

---

## Architecture

### Module Overview

| Module | Purpose | LOC | Tests |
|--------|---------|-----|-------|
| **parser.rs** | Parse game engine input | 308 | 5 ✅ |
| **output.rs** | Submit moves to stdout | 58 | 3 ✅ |
| **game_state.rs** | Grid and piece representation | 351 | 7 ✅ |
| **placement.rs** | Validate placements | 330 | 8 ✅ |
| **utils.rs** | Distance & adjacency helpers | 85 | 4 ✅ |
| **main.rs** | Game loop | 68 | - |

### Data Flow

```
Game Engine
    ↓
STDIN (formatted input)
    ↓
parser.rs
    ↓
game_state.rs (internal representation)
    ↓
placement.rs (validate moves)
    ↓
AI Strategy (Phase 4+)
    ↓
output.rs
    ↓
STDOUT (X Y\n format)
    ↓
Game Engine
```

---

## Development Progress

### Completed Phases

#### ✅ Phase 0: Setup & Infrastructure
- Cargo project initialization
- Git workflow with feature branches
- Comprehensive documentation (README, PROGRESS.md)
- CI-ready structure

#### ✅ Phase 1: Core Game Parser
- Parse player identification from engine
- Parse Anfield (grid) dimensions and cells
- Parse piece shapes and dimensions
- Implement game state structures
- **Status**: 768 lines, 15 tests, ✅ passing

#### ✅ Phase 3: Piece Placement Algorithm
- Boundary checking (piece fits in grid)
- Collision detection (opponent overlap)
- Territory validation (exactly 1 cell contact)
- Find all valid placements
- **Status**: 500 lines, 12 tests, ✅ passing

### Current Codebase Status

- **Total Lines**: 1,204 implementation
- **Unit Tests**: 27 (100% passing ✅)
- **Build Status**: ✅ Clean compilation
- **Test Coverage**: Parser, output, game state, placement, utilities

### Next Phase

#### ⏳ Phase 4: Basic AI Strategy
- Move evaluation function
- Greedy expansion heuristics
- Move ranking and selection
- Integration into game loop
- **ETA**: 2-3 days

See `PROGRESS.md` for detailed phase breakdown.

---

## Quick Start

### Prerequisites
- Rust 1.63+ (installed via [rustup](https://rustup.rs/))
- Docker (for testing with game engine)

### Build & Test

```bash
# Clone or navigate to project
cd filler

# Build debug version
cargo build

# Build release (optimized)
cargo build --release

# Run all tests
cargo test

# Run specific module tests
cargo test placement

# Check code without building
cargo check
```

### Testing Locally (Debug)

```bash
# Create test input file
cat > test_input.txt << 'EOF'
$$$ exec p1 : [test]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.
EOF

# Run with test input
./target/debug/filler < test_input.txt
```

---

## Docker Integration

### Setup

**1. Navigate to docker_image folder**
```bash
cd docker_image/
```

**2. Build Docker image**
```bash
docker build -t filler .
```

**3. Run container with mounted solution**
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

### Inside Container

**Copy your compiled binary**
```bash
# From inside container
cd /filler
cp /path/to/solution/filler solution/

# Or compile inside container (requires Rust in container)
cd solution && cargo build --release
```

**Run against opponents**

```bash
# Play against Bender (moderate)
./linux_game_engine -f maps/map00 -p1 solution/filler -p2 linux_robots/bender

# Play against H2D2
./linux_game_engine -f maps/map01 -p1 solution/filler -p2 linux_robots/h2_d2

# Play against WallE (easy, for testing)
./linux_game_engine -f maps/map02 -p1 solution/filler -p2 linux_robots/wall_e

# Challenge: Terminator (hard - optional)
./linux_game_engine -f maps/map01 -p1 solution/filler -p2 linux_robots/terminator

# Quiet mode (no throttling)
./linux_game_engine -f maps/map00 -p1 solution/filler -p2 linux_robots/bender -q

# With custom seed
./linux_game_engine -f maps/map00 -p1 solution/filler -p2 linux_robots/bender -s 12345

# Set timeout (seconds)
./linux_game_engine -f maps/map00 -p1 solution/filler -p2 linux_robots/bender -t 5
```

**Available Options**
```
-f, --file     Path to map
-p1            Player 1 binary path
-p2            Player 2 binary path
-q, --quiet    Quiet mode (no throttling)
-r, --refresh  Throttling mode
-s, --seed     Random seed
-t, --time     Timeout in seconds (default 10)
```

### Platform Specifics

**Linux**
- Use `linux_game_engine` and `linux_robots/`
- Standard Rust binary compilation

**Apple Silicon (M1/M2/M3)**
- Use `m1_game_engine` and `m1_robots/`
- Check native binary support

---

## Input/Output Format

### Input Format

The game engine sends input via stdin in this format:

```
$$$ exec p<player_num> : [<player_path>]
Anfield <width> <height>:
    <column_indices>
<row_num> <row_data>
...
Piece <width> <height>:
<piece_data>
```

### Cell Symbols

| Symbol | Meaning |
|--------|---------|
| `.` | Empty cell |
| `@` or `a` | Player 1 (@ = last placed, a = previous) |
| `$` or `s` | Player 2 ($ = last placed, s = previous) |

### Output Format

Submit your move via stdout:

```
X Y
```

Where `X` and `Y` are integer coordinates, separated by space, followed by newline.

**Example**: `7 2\n`

### Special Cases

If no valid placement exists, return fallback move:
```
0 0
```

The game engine will ignore invalid moves but continue the game.

---

## Code Examples

### Parsing Input

```rust
use parser::parse_game_input;

fn main() {
    match parse_game_input() {
        Ok(game_input) => {
            println!("Player: {}", game_input.player_number);
            println!("Anfield: {}×{}", 
                game_input.anfield.width, 
                game_input.anfield.height);
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}
```

### Validating Placements

```rust
use placement::{validate_placement, find_all_valid_placements};
use game_state::GameState;

let valid_placements = find_all_valid_placements(&game_state);

if valid_placements.is_empty() {
    // No valid moves available
    Move::fallback().submit()?;
} else {
    // Select and submit best placement
    let best = &valid_placements[0];
    Move::new(best.position.x, best.position.y).submit()?;
}
```

### Game State

```rust
use game_state::{GameState, CellState};

let my_positions = game_state.get_my_positions();
let territory_size = game_state.get_territory_size();

println!("My territory: {} cells", territory_size);
```

---

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test Suite

```bash
cargo test parser           # Parser tests
cargo test output           # Output tests
cargo test game_state       # Game state tests
cargo test placement        # Placement validation tests
cargo test utils            # Utility function tests
```

### Test Coverage

Current test coverage:

| Module | Tests | Status |
|--------|-------|--------|
| parser.rs | 5 | ✅ |
| output.rs | 3 | ✅ |
| game_state.rs | 7 | ✅ |
| placement.rs | 8 | ✅ |
| utils.rs | 4 | ✅ |
| **Total** | **27** | **✅** |

---

## Performance

### Benchmarks

| Operation | Time | Status |
|-----------|------|--------|
| Parse input | < 1ms | ✅ |
| Find all placements (20×15 grid) | < 50ms | ✅ |
| Validate single placement | < 1ms | ✅ |
| Full turn | < 100ms | ✅ |

### Optimization Targets (Phase 6)

- Move evaluation: < 100ms
- Large grid (100×100): < 1s
- Full turn: < 10s (game engine timeout)

---

## Known Issues & Limitations

### Phase 1-3 (Current)
- No intelligent AI (selects first valid placement)
- Limited move evaluation
- No performance optimization

### Will Be Fixed
- **Phase 4**: Basic AI strategy
- **Phase 5**: Advanced heuristics
- **Phase 6**: Performance optimization
- **Phase 7**: Docker integration testing

---

## Git Workflow

### Branch Strategy

```
main (production)
├── feat/basic-ai (Phase 4)
├── feat/advanced-ai (Phase 5)
├── feat/optimization (Phase 6)
└── [integration tests on main]
```

### Commit Format

```
<type>(<scope>): <description>

[optional body]
```

**Types**: `feat`, `fix`, `test`, `chore`, `docs`

**Example**:
```
feat(placement): add boundary checking for placement validation
```

---

## Debugging

### Debug Output

The binary writes debug information to stderr:

```bash
./target/debug/filler < input.txt 2> debug.log
```

### Enable Verbose Output

```bash
RUST_LOG=debug cargo run < input.txt
```

### Common Issues

**Parser Error**
- Verify input format matches specification
- Check column indices line is present
- Ensure row numbers are included

**Move Not Accepted**
- Verify output is `X Y\n` format (with newline)
- Check coordinates are valid positions
- Ensure piece doesn't overlap opponent territory

**Timeout**
- Current AI selects first valid move (very fast)
- Phase 4 AI will add evaluation logic
- Phase 6 will optimize for speed

---

## Contributing

This is a student project following professional development practices.

### Before Committing

```bash
cargo test           # All tests pass?
cargo build --release  # Compiles cleanly?
cargo check          # No warnings?
cargo fmt            # Code formatted?
```

### Commit Workflow

1. Create feature branch: `git checkout -b feat/feature-name`
2. Make changes with atomic commits
3. Add tests for new functionality
4. Merge to main: `git checkout main && git merge --no-ff feat/feature-name`

---

## Bonuses

### 1. Graphic Visualizer
Create a visual representation of the game state:
- Display grid with colored territories
- Show valid placement options
- Animate piece placement
- Display score/territory size

### 2. Terminator Beatdown
Implement advanced AI to defeat the Terminator robot:
- Implement predictive blocking
- Territory density analysis
- Offensive positioning
- Endgame strategies

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Game Specification](./PROGRESS.md#game-overview)
- Docker Documentation
- GitHub Copilot for code assistance

---

## License

This project is provided as-is for educational purposes.

---

## Development Timeline

| Phase | Topic | Status | ETA |
|-------|-------|--------|-----|
| 0 | Setup | ✅ | 1 day |
| 1 | Parser | ✅ | 1 day |
| 2 | Game State | ✅ | 1 day |
| 3 | Placement | ✅ | 1 day |
| 4 | Basic AI | ⏳ | 2-3 days |
| 5 | Advanced AI | ⏳ | 4-5 days |
| 6 | Optimization | ⏳ | 2-3 days |
| 7 | Docker Testing | ⏳ | 2-3 days |

**Total Estimated**: ~18-21 days (with bonuses)

---

## Status Summary

✅ **Foundation Built**
- Parser, game state, placement validation complete
- 27 unit tests passing
- Professional workflow established

⏳ **In Development**
- Phase 4: AI strategy implementation

🚀 **Next Steps**
1. Implement basic move evaluation
2. Add greedy expansion strategy
3. Test against provided robots
4. Iterate on heuristics
5. Optimize for performance

---

**Last Updated**: December 9, 2025  
**Project Status**: Phase 3 Complete, Phase 4 Ready  
**Ready for**: Docker testing, competitive play, optimization

