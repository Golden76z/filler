#!/usr/bin/env bash
set -u
FILLER="${FILLER:-./solution/target/release/filler}"
MAP="${MAP:-maps/map01}"
GAMES_PER_MATCH=${GAMES_PER_MATCH:-5}

wins=0
for i in $(seq 1 "$GAMES_PER_MATCH"); do
    if [ $((i % 2)) -eq 1 ]; then
        side="p1"
        win_marker="Player1 won"
        result=$(./linux_game_engine -q -f "$MAP" -p1 "$FILLER" -p2 linux_robots/terminator 2>&1 | tail -n 5)
    else
        side="p2"
        win_marker="Player2 won"
        result=$(./linux_game_engine -q -f "$MAP" -p1 linux_robots/terminator -p2 "$FILLER" 2>&1 | tail -n 5)
    fi
    scoreline=$(echo "$result" | grep -E "Player[12] \(" | tr '\n' ' ')
    if echo "$result" | grep -q "$win_marker"; then
        wins=$((wins + 1))
        echo "  [terminator #$i map=$MAP side=$side] WIN   $scoreline"
    else
        echo "  [terminator #$i map=$MAP side=$side] LOSS  $scoreline"
    fi
done

echo "terminator on $MAP -> $wins/$GAMES_PER_MATCH wins"
