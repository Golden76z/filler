#!/usr/bin/env bash
# Runs the audit matrix (5 games per opponent, alternating sides) inside the
# container and prints win counts. Expects to be run from /filler.
set -u

FILLER="${FILLER:-./solution/target/release/filler}"
GAMES_PER_MATCH=${GAMES_PER_MATCH:-5}

run_match() {
    local label="$1"
    local map="$2"
    local opp="$3"
    local wins=0
    for i in $(seq 1 "$GAMES_PER_MATCH"); do
        if [ $((i % 2)) -eq 1 ]; then
            side="p1"
            win_marker="Player1 won"
            result=$(./linux_game_engine -q -f "$map" -p1 "$FILLER" -p2 "linux_robots/$opp" 2>&1 | tail -n 5)
        else
            side="p2"
            win_marker="Player2 won"
            result=$(./linux_game_engine -q -f "$map" -p1 "linux_robots/$opp" -p2 "$FILLER" 2>&1 | tail -n 5)
        fi
        if echo "$result" | grep -q "$win_marker"; then
            wins=$((wins + 1))
            scoreline=$(echo "$result" | grep -E "Player[12] \(" | tr '\n' ' ')
            echo "  [$label #$i side=$side] WIN   $scoreline"
        else
            scoreline=$(echo "$result" | grep -E "Player[12] \(" | tr '\n' ' ')
            echo "  [$label #$i side=$side] LOSS  $scoreline"
        fi
    done
    echo "$label vs $opp on $map -> $wins/$GAMES_PER_MATCH wins"
    echo ""
}

echo "===== map00 vs wall_e ====="
run_match "wall_e" maps/map00 wall_e

echo "===== map01 vs h2_d2 ====="
run_match "h2_d2" maps/map01 h2_d2

echo "===== map02 vs bender ====="
run_match "bender" maps/map02 bender
