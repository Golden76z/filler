# Filler — Audit script

Step-by-step commands to walk through the audit. Each section quotes the
audit question, then gives the exact commands and what to look for in the
output. Run from a shell where `docker` works.

> Two helper scripts are used:
> - **Windows / PowerShell**: `solution\docker-filler.ps1`
> - **Linux / macOS / Git Bash**: `solution/docker-filler.sh`
>
> Both forward a single bash command into the `filler` Docker image with
> `solution/` mounted at `/filler/solution`. Pick whichever matches your
> shell. From this point on, commands are shown in both forms; pick one.

---

## 0. One-time setup — build the image

> **Audit:** *Try to run the command `./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator` inside the container.*
> *Can you confirm that the student was able to create the image and container correctly?*

**PowerShell:**

```powershell
cd C:\Users\Damien\Desktop\dev\rust\filler\solution
.\docker-filler.ps1 -Build -Command 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator | tail -n 5'
```

**Bash:**

```bash
cd solution
./docker-filler.sh --build -- 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator | tail -n 5'
```

Expected last lines (the exact winner doesn't matter — it just proves the
image and the engine work together):

```
seed: <number>
Player1 (linux_robots/bender): <score>
Player2 (linux_robots/terminator): <score>
Player2 won!
```

---

## 1. Run the student player against another player

> **Audit:** *Try to run the student player against one of our players.*
> *Can you confirm that the project runs correctly?*

**PowerShell:**

```powershell
.\docker-filler.ps1 -Command 'cd /filler && ./linux_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 linux_robots/wall_e | tail -n 5'
```

**Bash:**

```bash
./docker-filler.sh -- 'cd /filler && ./linux_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 linux_robots/wall_e | tail -n 5'
```

Expected output ends with `Player1 won!` (the student is `p1` here).

---

## 2. Pieces overlap exactly one cell

> **Audit:** *Can you confirm that the student player is placing the pieces correctly with the overlapping of just on cell?*

Two ways to demonstrate.

### 2a. By the rules of the engine

If the engine accepts every move the student makes and the game progresses
to a normal end (no `error` or `invalid placement` messages), the
exactly-one-overlap rule is being enforced — the engine itself rejects
moves that violate it. Tail a verbose run:

**PowerShell:**

```powershell
.\docker-filler.ps1 -Command 'cd /filler && ./linux_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 linux_robots/wall_e 2>&1 | grep -iE "error|invalid|won" | tail -n 10'
```

**Bash:**

```bash
./docker-filler.sh -- 'cd /filler && ./linux_game_engine -f maps/map00 -p1 ./solution/target/release/filler -p2 linux_robots/wall_e 2>&1 | grep -iE "error|invalid|won" | tail -n 10'
```

Expected: only the final `Player? won!` line; no `error` or `invalid`
mentions for `Player1`.

### 2b. By the source code & tests

The rule lives in `src/placement.rs`:

```rust
pub fn is_valid_placement(player, field, piece, x, y) -> bool {
    // ... counts `own` and `opp` overlaps ...
    opp == 0 && own == 1
}
```

The unit tests cover the rule from every angle:

- `requires_exactly_one_overlap_with_own`
- `rejects_two_own_overlaps`
- `rejects_opponent_overlap`
- `ignores_piece_empty_cells_for_overlap_rules`
- `player2_symbols`

These are run in section 4.

---

## 3. The three required match-ups (5 games each, alternating sides)

> **Audit:** *Run `./game_engine -f maps/map00 -p1 <student> -p2 robots/wall_e` five times changing the position of the players each time so that the student player can be the p1 and the p2. Can you confirm that the student player won at least 4 out of 5 times?*
> *Same for map01 vs h2_d2 and map02 vs bender.*

A single helper, `solution/audit.sh`, runs all 15 games (5 games × 3
match-ups), alternates the student between `p1` and `p2`, and tallies wins.

**PowerShell:**

```powershell
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && bash ./solution/audit.sh'
```

**Bash:**

```bash
./docker-filler.sh -- 'cd /filler/solution && cargo build --release && cd /filler && bash ./solution/audit.sh'
```

Expected tail (the score lines and exact margins will vary):

```
wall_e vs wall_e on maps/map00 -> 5/5 wins
h2_d2 vs h2_d2 on maps/map01 -> 5/5 wins
bender vs bender on maps/map02 -> 5/5 wins
```

All three lines must be ≥ 4/5. (Empirically: 5/5 each in repeated runs.)

---

## 4. Unit tests

> **Audit:** *Do all tests pass without errors?*
> *Are there specific tests for Input Parsing? Placement Validation? Boundary Detection?*

**PowerShell:**

```powershell
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo test --release'
```

**Bash:**

```bash
./docker-filler.sh -- 'cd /filler/solution && cargo test --release'
```

Expected: every test result line says `test result: ok.` and the final
summary is `16 passed; 0 failed` for the lib and `4 passed; 0 failed`
for the integration tests. Coverage by audit topic:

| Audit topic | Tests |
|---|---|
| **Input parsing** | `parser::tests::parses_player_anfield_dimensions_and_piece`, `parser::tests::parses_without_column_header_line`, `parser::tests::second_turn_reuses_player_hint`, `log_replay::tests::parses_two_consecutive_frames_and_skips_non_anfield_lines`, `log_replay::tests::handles_log_without_column_header` |
| **Placement validation** | `placement::tests::requires_exactly_one_overlap_with_own`, `rejects_two_own_overlaps`, `rejects_opponent_overlap`, `ignores_piece_empty_cells_for_overlap_rules`, `player2_symbols` |
| **Boundary detection** | `placement::tests::rejects_out_of_bounds`, integration `rejects_placements_outside_bounds_explicitly`, integration `placement_never_extends_beyond_grid` |
| **Output format** | `output::tests::exact_format_with_trailing_newline`, `output::tests::pass_format` |
| **End-to-end** | integration `full_round_trip_first_turn`, `prefers_moves_pointing_toward_opponent` |
| **Strategy sanity** | `strategy::tests::picks_move_toward_opponent`, `handles_no_opponent_yet`, `returns_none_when_no_valid_placement` |

---

## 5. Good practices

> **Audit:** *Does the code obey the good practices?*

Spot-checks you can run:

```powershell
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release 2>&1 | grep -E "warning|error" || echo "no warnings or errors"'
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo fmt --check && echo "rustfmt clean"'
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo clippy --release -- -D warnings 2>&1 | tail -n 20 || true'
```

(Bash variant is identical via `docker-filler.sh`.)

Expected: a clean build with no warnings. (`cargo fmt` and `cargo clippy`
are nice-to-haves and may not be installed in the rust:1.63-buster image
— they don't gate the audit.)

The repo also follows the standard Rust layout: `src/lib.rs` for the
library, `src/main.rs` for the robot binary, `src/bin/` for additional
binaries, `tests/` for integration tests, no `unsafe`, no `unwrap` on
runtime input, modules carry doc comments.

---

## 6. Bonus — visualizer

> **Audit:** *Did the student create a visualizer for the project?*

Two visualizers ship:

### 6a. ANSI terminal replay (built in Docker)

```powershell
.\docker-filler.ps1 -Command 'cd /filler && ./linux_game_engine -f maps/map00 -p1 linux_robots/wall_e -p2 ./solution/target/release/filler > ./solution/game.log'
.\docker-filler.ps1 -Interactive -Command 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
```

You should see each board frame redraw in colour, ending with the final
score line.

### 6b. Native GUI replay (eframe/egui)

Needs Rust on the host once (`https://rustup.rs/`).

```powershell
cd solution
.\build-gui.ps1
.\target\release\visualizer-gui.exe .\game.log
```

A window opens with the board, play/pause, speed slider, scrub bar, and
live P1/P2 cell counts. See `DETAILS.md` for screenshots-equivalent walk-through.

---

## 7. Bonus — beat the terminator

> **Audit:** *Run `./game_engine -f maps/<map of your choice> -p1 <student> -p2 robots/terminator` five times … Can you confirm that the student player won at least 4 out of 5 times?*

Use `map02`, where the bot consistently hits ≥ 4/5:

**PowerShell:**

```powershell
.\docker-filler.ps1 -Command 'cd /filler/solution && cargo build --release && cd /filler && MAP=maps/map02 bash ./solution/audit_terminator.sh'
```

**Bash:**

```bash
./docker-filler.sh -- 'cd /filler/solution && cargo build --release && cd /filler && MAP=maps/map02 bash ./solution/audit_terminator.sh'
```

Expected final line:

```
terminator on maps/map02 -> 4/5 wins
```

(or 5/5; the strong matchup is `map02`, which the README documents).

---

## Summary

| Audit section | Pass criterion | Command |
|---|---|---|
| Image / container builds | `Player? won!` printed | `docker-filler -Build -Command '... | tail -n 5'` |
| Project runs vs another bot | game ends, `won!` | `docker-filler -Command '... | tail -n 5'` |
| Exactly-one-overlap rule | no `error`/`invalid` for student | `... | grep error` |
| map00 vs wall_e ≥ 4/5 | `5/5 wins` line | `bash ./solution/audit.sh` |
| map01 vs h2_d2 ≥ 4/5 | `5/5 wins` line | same |
| map02 vs bender ≥ 4/5 | `5/5 wins` line | same |
| Tests pass | `0 failed` in every suite | `cargo test --release` |
| Visualizer bonus | window opens, frames animate | `build-gui.ps1` + run |
| Terminator bonus ≥ 4/5 | `4/5 wins` line | `bash ./solution/audit_terminator.sh` |
