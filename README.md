# Filler

A Rust implementation of the Filler bot. It reads game state from stdin (provided
by `game_engine`), picks a placement and writes `X Y\n` to stdout, until the
engine stops feeding turns.

## Layout

```
solution/
├── Cargo.toml
├── src/
│   ├── lib.rs             # public re-exports of modules
│   ├── main.rs            # robot binary (read stdin, write moves)
│   ├── game_state.rs      # Player / Anfield / Piece / Turn types
│   ├── parser.rs          # parses engine stdin into a Turn
│   ├── placement.rs       # validation: bounds + exactly-one overlap rule
│   ├── strategy.rs        # BFS-distance heuristic toward opponent
│   ├── output.rs          # `X Y\n` formatting
│   └── bin/
│       └── visualizer.rs  # bonus ANSI visualizer for game logs
└── tests/
    └── integration.rs     # end-to-end parse → choose → format
```

## Build & run (inside the provided Docker image)

### Cross-platform helpers (recommended)

From `solution/` you can avoid hand-typed volume paths:

- **Windows (PowerShell):** `docker-filler.ps1`
- **Linux / macOS / Git Bash:** `docker-filler.sh` (may need `chmod +x docker-filler.sh` once)

Record `game.log` (no TTY needed):

```powershell
cd solution
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'
```

Replay with the ANSI visualizer (needs a TTY — use `-Interactive`):

```powershell
.\docker-filler.ps1 -Interactive -Command 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
```

Same idea on bash:

```sh
cd solution
./docker-filler.sh -- 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'
./docker-filler.sh -it -- 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
```

First-time (or after changing the Dockerfile), add `-Build` / `--build` before the command.

If PowerShell blocks scripts, run once for your user: `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`.

### Manual Docker (any shell)

From the repository root that contains `docker_image/`:

```sh
cd docker_image
docker build -t filler .
docker run -v "$(pwd)/../solution":/filler/solution -it filler
```

Inside the container:

```sh
cd /filler/solution
cargo build --release
./target/release/filler   # robot binary, reads stdin

cd /filler
./linux_game_engine -f maps/map01 \
    -p1 ./solution/target/release/filler \
    -p2 linux_robots/wall_e
```

On M1 Macs swap `linux_*` for `m1_*`.

## Tests

```sh
cd /filler/solution
cargo test
```

The suite covers:

- **Input parsing** — engine handshake line, Anfield header/dimensions, row prefix
stripping, piece header/dimensions, second-turn re-use of the player hint.
- **Placement validation** — exactly-one-own-cell rule, opponent-overlap rejection,
two-own-overlap rejection, player 2 symbols, ignoring `.` cells of the piece.
- **Boundary detection** — out-of-bounds X/Y refused.
- **Output formatting** — exact `X Y\n` shape (and `0 0\n` pass).
- **End-to-end** — parsed turn → chosen placement is valid and within bounds.

## Strategy

A multi-source BFS computes the 8-connected (Chebyshev) distance from every
opponent cell. Each candidate placement is scored lexicographically by:

1. The minimum distance from any newly added piece cell to the opponent.
2. The sum of those distances.
3. A small edge-hugging penalty as tie-breaker.

Lower is better, so the bot constantly drives toward the opponent. Cells already
part of own territory are skipped when scoring (only the *new* cells matter).
This beats `wall_e`, `h2_d2` and `bender` reliably (5/5 in the audit script) and
also wins ≥ 4/5 against `terminator` on `maps/map02`.

If no opponent has been seen yet, the bot just plays the first legal placement.
If none exists, it emits `0 0\n` as a pass.

## Reproducing the audit

The repository ships with two helper scripts:

- `solution/audit.sh` — 5 games on each of `map00 vs wall_e`, `map01 vs h2_d2`,
`map02 vs bender`, alternating sides each game.
- `solution/audit_terminator.sh` — same shape for the bonus terminator match
(`MAP=maps/map02 bash ./solution/audit_terminator.sh` to use the strong map).

Run them from inside the container at `/filler`:

```sh
cargo build --release --manifest-path solution/Cargo.toml
bash ./solution/audit.sh
MAP=maps/map02 bash ./solution/audit_terminator.sh
```

## Bonus: visualizers

Two replay UIs are provided.

### 1. ANSI terminal replay (`visualizer`, built in Docker)

Pipe a live game, or replay a captured log:

```sh
./linux_game_engine -f maps/map00 \
    -p1 linux_robots/wall_e \
    -p2 ./solution/target/release/filler \
    | ./solution/target/release/visualizer --delay 150

./linux_game_engine ... > game.log
./solution/target/release/visualizer game.log --delay 120
```

### 2. Native GUI replay (`visualizer-gui`, eframe/egui, built on the host)

A real desktop window with play / pause, speed slider, scrub bar, step
buttons, P1/P2 cell counts, and drag-and-drop of `game.log` files. Keyboard
shortcuts: `Space` play/pause, `←` / `→` step, `Home` / `End` first / last.

The GUI is gated behind the `gui` Cargo feature and is only built when
explicitly asked, so the Docker build of the audit binary keeps no GUI
dependencies.

Because a window needs a display, the GUI must be built on the **host** OS:

**Windows:**

```powershell
# one-time: install Rust from https://rustup.rs/  (then reopen PowerShell)
cd solution
.\build-gui.ps1
.\target\release\visualizer-gui.exe .\game.log
```

**Linux / macOS:**

```sh
cd solution
cargo build --release --features gui --bin visualizer-gui
./target/release/visualizer-gui ./game.log
```

You can also launch it with no argument and drop a `game.log` on the window.

Recording a game log to feed the GUI (using the cross-platform docker helper):

```powershell
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'
```

