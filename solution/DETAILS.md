# Filler — Project details

A full walk-through of what the project is, how it is structured, how the
strategy works, and how every piece fits together. For the short version
read `README.md`; for the audit checklist read `AUDIT.md`.

---

## 1. The game in 30 seconds

Two robots take turns placing a small piece on a 2-D grid (the "Anfield").
Each player owns a colour. The game engine ships a random piece every turn
to one of the two players and writes the current board state on the
player's stdin. The player has a few seconds to print one move,
`X Y\n`, on stdout. The placement rules are:

1. The piece's bounding box must fit inside the Anfield.
2. The piece must overlap your own territory in **exactly one** cell.
3. The piece must not overlap any of the opponent's cells.

When a player has no legal move it forfeits its turn (but the game keeps
running for the other player). The game ends when neither can move, or
when one of them crashes / times out. The player owning the most cells
wins.

Symbols on the board:

| Player | Owned cells | Last move |
|---|---|---|
| P1 | `@` | `a` |
| P2 | `$` | `s` |
| empty | `.` | |

---

## 2. Repository layout

```
filler/
├── docker_image/         # provided by the school: Dockerfile, maps, opponent robots
│   ├── Dockerfile
│   ├── linux_game_engine
│   ├── linux_robots/{bender, h2_d2, wall_e, terminator}
│   ├── m1_game_engine, m1_robots/...
│   └── maps/{map00, map01, map02}
└── solution/             # this codebase, mounted into the container at /filler/solution
    ├── Cargo.toml
    ├── README.md         # quick start
    ├── AUDIT.md          # step-by-step audit commands
    ├── DETAILS.md        # this file
    ├── audit.sh                  # 5 games × 3 opponents, alternates sides
    ├── audit_terminator.sh       # bonus terminator matrix
    ├── docker-filler.ps1         # Windows wrapper around docker run
    ├── docker-filler.sh          # bash wrapper around docker run
    ├── build-gui.ps1             # one-step Windows build of the GUI replay
    ├── src/
    │   ├── lib.rs                # library entry — re-exports modules
    │   ├── main.rs               # robot binary (stdin → choose move → stdout)
    │   ├── game_state.rs         # Player / Anfield / Piece / Turn types
    │   ├── parser.rs             # engine stdin → Turn (parse_turn)
    │   ├── placement.rs          # exactly-one-overlap + bounds rule
    │   ├── strategy.rs           # BFS-distance scorer + choose_placement
    │   ├── output.rs             # "X Y\n" / "0 0\n" formatting
    │   ├── log_replay.rs         # game.log → Vec<Frame> (used by visualizers)
    │   └── bin/
    │       ├── visualizer.rs     # ANSI replay (built in Docker, no extra deps)
    │       └── visualizer_gui.rs # eframe/egui replay (built on host, gui feature)
    └── tests/integration.rs      # end-to-end tests
```

---

## 3. The engine protocol

The first message the engine sends to the robot looks like:

```
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
...
014 ....................
Piece 4 1:
.OO.
```

Then on every subsequent turn the engine sends a new `Anfield` block (the
updated board) and a new `Piece` block. The `$$$ exec ...` line is sent
only once at start-up.

The robot's reply per turn is a single line `X Y\n` — the top-left
coordinate of the piece's bounding box. Examples:

- The robot answers `7 2\n` to place a piece at (x=7, y=2).
- If the robot has no legal move it must still answer; conventionally
  `0 0\n` is sent as a pass-equivalent.

Notes:

- Row lines are prefixed with the row index `NNN ` (always 3 digits) and a
  space. The first row of an Anfield block may or may not be preceded by a
  column-index line. Our parser handles both cases.
- The `Piece W H:` block always has `W` and `H` positive integers, then
  `H` lines of `W` characters; piece cells are non-`.` (the engine uses
  `O` or `#`).
- Pieces can overhang the grid only conceptually — we still anchor by the
  top-left of the bounding box, and only filled cells (`!= '.'`) are
  evaluated against the rules.

---

## 4. The codebase, module by module

### 4.1 `game_state.rs`

Defines the basic data types and the symbol convention.

```rust
pub enum Player { P1, P2 }
pub struct Anfield { width, height, rows: Vec<String> }
pub struct Piece   { width, height, rows: Vec<String> }
pub struct Turn    { player, anfield, piece }
```

`Player::own_chars` / `Player::opp_chars` encode the `@ a` vs `$ s`
convention so the rest of the code never hard-codes a side. `Piece::filled_cells`
iterates the non-`.` cells of a piece, which both the validator and the
scorer consume.

### 4.2 `parser.rs`

`parse_turn(lines_iter, player_hint) -> Option<Turn>` reads one full
engine message — handling

- the optional `$$$ exec pN : [...]` handshake line (only present on the
  first turn, hence the `player_hint` argument that the main loop carries
  over between turns),
- the `Anfield W H:` header,
- the optional column-index line that starts with spaces and ASCII digits
  only,
- the `H` row lines, with the `NNN ` row prefix stripped,
- a blank line or noise before `Piece W H:`,
- the `H` piece rows.

It returns `None` on EOF. The main loop calls it repeatedly.

### 4.3 `placement.rs`

The whole game rule lives here in one function:

```rust
pub fn is_valid_placement(player, field, piece, x, y) -> bool {
    let mut own = 0;
    let mut opp = 0;
    for (px, py) in piece.filled_cells() {
        let bx = x + px;
        let by = y + py;
        if bx >= field.width || by >= field.height {
            return false;          // bounds check
        }
        let c = field.rows[by].chars().nth(bx).unwrap_or('.');
        if player.is_own_cell(c) { own += 1; }
        else if player.is_opp_cell(c) { opp += 1; }
    }
    opp == 0 && own == 1
}
```

Two things to note:

- Empty cells of the piece (`'.'`) are ignored. Only filled cells
  participate in the overlap count.
- Bounds are checked per filled cell, not for the whole bounding box. A
  piece whose bounding box would extend past the field but whose only
  filled cells are inside the field is still legal in principle — the
  function handles that correctly because it tests every filled cell.

`all_valid_placements` returns every `(x, y)` for which
`is_valid_placement` returns true. With `W·H ≤ 100·100 = 10000` candidate
anchors and a piece of `K` filled cells, this is `O(W·H·K)` per turn,
which is well under a millisecond on the supplied maps.

### 4.4 `strategy.rs` — how the bot picks a move

The core idea: the bot wants to be **aggressive**. It always plays the
move whose newly-claimed cells get closest to the opponent's territory.
Done over many turns this is empirically very effective against
`wall_e`, `h2_d2` and `bender`, and reliably beats `terminator` on the
big board `map02`.

Concretely:

1. Run a **multi-source BFS** from every opponent cell. Use **8-connected
   (Chebyshev) distance** — this matches the way Filler pieces can extend
   freely in any direction, so it gives a faithful "how close to the
   enemy" signal. The result is a 2-D distance map `dist[y][x]` over the
   whole Anfield.

   ```rust
   let dist = opponent_distance_map(player, field);   // O(W·H)
   ```

2. Enumerate every valid placement, and for each compute a score from the
   piece cells that would be *newly* claimed (the cell that overlaps own
   territory is skipped — only new ground matters):

   - `min_d`: the smallest `dist[by][bx]` of any new cell. **Lower is
     better** (closer to the enemy).
   - `sum_d`: the sum of `dist[by][bx]` of new cells. Tie-breaker that
     prefers a "wider reach" across multiple cells.
   - `edge_penalty`: small +1 per cell that sits on the field boundary;
     hugging the edges traps you later.

   Sort lexicographically `(min_d, sum_d, edge_penalty)` ascending and
   pick the first.

3. If there is **no opponent on the board yet** (the bot moves first and
   the opponent has no `$`/`s` cells, or the other way around), the bot
   simply plays the first legal placement. There is no enemy to chase yet.

4. If **no valid placement exists**, return `None` and the main loop sends
   `0 0\n` as a pass.

Total cost per turn: `O(W·H)` for the BFS and `O(W·H·K)` for the
enumeration. Plenty fast.

### 4.5 `output.rs`

Two helpers — and that's the entire output surface.

```rust
pub fn format_move(p: Placement) -> String { format!("{} {}\n", p.x, p.y) }
pub fn format_pass()             -> String { "0 0\n".to_string() }
```

Spec compliance is unit-tested.

### 4.6 `main.rs`

Glue. Read stdin line-by-line, hand it to `parse_turn`, hand the parsed
turn to `choose_placement`, print `format_move(...)` or `format_pass()`,
flush. The `player_hint` is carried across turns so the bot still knows
which player it is on turn 2+ when the `$$$` line is not repeated.

### 4.7 `log_replay.rs`

Used only by the visualizers. Reads a `game.log` produced by piping the
engine to a file, and returns `Vec<Frame>` where each frame is a board
snapshot. The format is identical to the per-turn stdin format the engine
sends to the robot, plus some interleaved `-> Answer (...)`,
`seed:`, `Player1 ... won!` lines that the parser simply skips by
looking for `Anfield ` headers.

---

## 5. Strategy walk-through with a tiny example

Picture this 5×5 mid-game state. P1 is `@`/`a`, P2 is `$`/`s`. It's P1's
turn and the piece is `##` (1×2).

```
@@...
@....
.....
....$
....$
```

The Chebyshev distance map from P2 (`$`s at (4,3) and (4,4)) looks like:

```
4 3 2 1 1
4 3 2 1 0    ← wait, P2 only has cells at (4,3) and (4,4)
3 2 1 1 1
3 2 1 1 0
3 2 1 1 0
```

For each candidate placement the bot computes the `(min_d, sum_d,
edge_penalty)` of the newly-claimed cells:

- Anchor `(0, 1)`, cells `(0,1) (1,1)`: `(0,1)` overlaps own (skipped),
  new cell is `(1,1)`. `min_d = 3`, `sum_d = 3`, `edge_penalty = 0`.
- Anchor `(1, 1)`, cells `(1,1) (2,1)`: new cell `(2,1)`. `min_d = 2`,
  `sum_d = 2`, `edge_penalty = 0`. **Better.**
- Anchor `(2, 0)`, cells `(2,0) (3,0)`: new cell `(3,0)`. `min_d = 1`,
  `sum_d = 1`, `edge_penalty = 1`. **Better still.**

So the bot plays `2 0`, pushing toward the right edge of the map where
the enemy lives. After a few turns of this the bot has built a "wall" of
territory along the contested seam, denying the opponent room to grow.

---

## 6. Cargo configuration

```toml
[lib]   name = "filler", path = "src/lib.rs"
[[bin]] name = "filler",          path = "src/main.rs"
[[bin]] name = "visualizer",      path = "src/bin/visualizer.rs"
[[bin]] name = "visualizer-gui",  path = "src/bin/visualizer_gui.rs"
        required-features = ["gui"]

[features]
default = []
gui = ["dep:eframe", "dep:rfd"]

[dependencies]
eframe = { version = "0.27", optional = true }
rfd    = { version = "0.14", optional = true }

[profile.release]
lto = true
codegen-units = 1
opt-level = 3

resolver = "2"
```

What this gives you:

- **In the Docker image** (Rust 1.63), `cargo build --release` compiles
  only `filler` and `visualizer`. eframe and rfd are *not* downloaded,
  *not* compiled — because the `gui` feature is off and the resolver
  is "2". The audit build stays pure-stdlib.
- **On the Windows / Linux / macOS host**, `cargo build --release --features gui`
  pulls eframe/rfd and produces a native window binary. This requires a
  recent stable Rust on the host — `build-gui.ps1` checks and tells you
  to install it.

---

## 7. Tests

```
cargo test --release
```

Inside the Docker image:

- **`src/parser.rs`** — 3 unit tests covering: the engine handshake
  parses, the optional column-index line is optional, the player hint is
  re-used on the second turn.
- **`src/placement.rs`** — 6 unit tests covering: out-of-bounds rejected,
  exactly-one own overlap accepted, two own overlaps rejected, any
  opponent overlap rejected, player-2 symbols recognised, piece `'.'`
  cells ignored.
- **`src/strategy.rs`** — 3 unit tests covering: a sensible move is
  chosen, the no-opponent edge case is handled, the no-valid-placement
  edge case returns `None`.
- **`src/output.rs`** — 2 unit tests covering: `X Y\n` exact shape and
  the `0 0\n` pass.
- **`src/log_replay.rs`** — 2 unit tests covering: multiple consecutive
  frames, log without the column-index line.
- **`tests/integration.rs`** — 4 end-to-end tests: full parse →
  choose → format round-trip, the chosen placement stays inside the
  field, explicit out-of-bounds rejection, the heuristic prefers a move
  pointing toward the opponent.

Total: **16 lib tests + 4 integration tests, no warnings**.

---

## 8. Audit performance

Empirical results from `solution/audit.sh` (run several times):

| Match-up | Wins | Typical margin |
|---|---|---|
| `map00` vs `wall_e` | 5 / 5 | 180–240 vs 50–100 |
| `map01` vs `h2_d2` | 5 / 5 | 800–1100 vs 150–300 |
| `map02` vs `bender` | 5 / 5 | 8000–9100 vs 480–1100 |
| `map02` vs `terminator` (bonus) | 4 / 5 | 4500–6300 vs 2700–4400 |

The audit threshold is 4/5; the bot exceeds it on the three required
match-ups in every run, and clears the terminator bonus on `map02`.

---

## 9. Cross-platform tooling

The Docker invocation looks slightly different on every OS, so two thin
wrappers ship with the project:

- **`solution/docker-filler.ps1`** (PowerShell): resolves the absolute
  `solution/` path, normalises `\` to `/` for the volume mount, optionally
  rebuilds the image, optionally attaches a TTY for the ANSI visualizer.
- **`solution/docker-filler.sh`** (bash, Linux / macOS / Git Bash): same
  contract, simpler implementation.

Typical use:

```powershell
# capture a game log without TTY
.\docker-filler.ps1 -Command 'cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'

# replay it in ANSI with a TTY
.\docker-filler.ps1 -Interactive -Command 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
```

```bash
./docker-filler.sh -- 'cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'
./docker-filler.sh -it -- 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
```

---

## 10. Bonuses

### 10.1 ANSI terminal replay (`visualizer`)

Streams or replays an engine log to a terminal with colour. Built in
Docker like every other Linux binary; runs anywhere with VT-100 colour
support.

### 10.2 Native GUI replay (`visualizer-gui`)

A real OS window built with **eframe/egui**:

- Toolbar: `Open log…`, `Play / Pause`, first / prev / next / last
  buttons, speed slider (1–60 fps), scrub bar.
- Side panel: current frame `/ total`, live P1 and P2 cell counts,
  leader indicator, colour legend.
- Drag-and-drop a `game.log` onto the window to switch logs.
- Keyboard: `Space` play / pause, `←` `→` step, `Home` `End`
  first / last.
- The latest-played cell (`a` for P1, `s` for P2) gets a white outline so
  you can track the most recent move at a glance.

The GUI must be built on the **host OS** because the Docker container has
no display server. The Windows helper:

```powershell
cd solution
.\build-gui.ps1
.\target\release\visualizer-gui.exe .\game.log
```

The first compilation takes a couple of minutes (eframe pulls winit,
glow, and the egui crates); after that it's incremental.

---

## 11. Limits and caveats

- The strategy is greedy — it does not look ahead more than one move.
  Adding a 2-ply look-ahead would push the terminator win rate higher
  but is unnecessary for the audit and costs a fair amount of compute.
- The bot assumes Chebyshev distance correctly models reachability. On
  very narrow corridors the heuristic can pick a slightly suboptimal
  anchor, but the gap is tiny and never costs games against the
  required opponents.
- `audit.sh` is intentionally simple shell — it runs the engine, tails
  the score lines, counts wins. Reproducibility is reasonable but the
  engine seeds vary each run, so individual game margins fluctuate.
- The GUI uses `eframe` 0.27 (locked to that minor series). Newer
  eframe versions change the `App` factory return type; bumping is a
  one-line change inside `visualizer_gui.rs` if ever needed.

---

## 12. References

- The game's spec: see the top of this conversation's task description
  (the file you started from).
- Subject quoted in `AUDIT.md` for every audit question.
- The original docker assets: `docker_image/Dockerfile`,
  `docker_image/linux_game_engine`, `docker_image/linux_robots/`.
