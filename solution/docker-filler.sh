#!/usr/bin/env bash
# Run bash -c '<command>' inside the `filler` Docker image with this `solution/` mounted at `/filler/solution`.
# Works on Linux, macOS, and Git Bash on Windows (Docker Desktop).
#
# Examples (note: keep the in-container command in ONE quoted string so && survives):
#   chmod +x docker-filler.sh   # once
#   ./docker-filler.sh --build -- 'cd /filler/solution && cargo build --release'
#   ./docker-filler.sh -- 'cd /filler/solution && cargo build --release && cd /filler && ./linux_game_engine -f maps/map02 -p1 ./solution/target/release/filler -p2 linux_robots/bender > ./solution/game.log'
#   ./docker-filler.sh -it -- 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
set -euo pipefail

IMAGE="${IMAGE:-filler}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
DOCKER_CONTEXT="${REPO_ROOT}/docker_image"

if [[ ! -d "${DOCKER_CONTEXT}" ]]; then
  echo "error: expected docker context at: ${DOCKER_CONTEXT}" >&2
  exit 1
fi

DO_BUILD=0
TTY_ARGS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build)
      DO_BUILD=1
      shift
      ;;
    -it)
      TTY_ARGS=(-i -t)
      shift
      ;;
    --)
      shift
      break
      ;;
    -h | --help)
      sed -n '1,12p' "$0" >&2
      exit 0
      ;;
    *)
      break
      ;;
  esac
done

if [[ "${DO_BUILD}" -eq 1 ]]; then
  docker build -t "${IMAGE}" "${DOCKER_CONTEXT}"
fi

if [[ $# -eq 0 ]]; then
  cat <<'EOF' >&2
error: missing command (pass one quoted bash script for inside the container).

Examples:
  ./docker-filler.sh --build -- 'cd /filler/solution && cargo build --release'
  ./docker-filler.sh -it -- 'cd /filler && ./solution/target/release/visualizer /filler/solution/game.log --delay 80'
EOF
  exit 2
fi

CMD="$*"
exec docker run --rm --entrypoint bash "${TTY_ARGS[@]}" -v "${SCRIPT_DIR}:/filler/solution" "${IMAGE}" -c "${CMD}"
