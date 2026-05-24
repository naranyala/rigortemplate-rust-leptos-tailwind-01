#!/bin/bash
set -e

# ============================================
# Leptos CSR Dashboard — Dev Pipeline
# ============================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}  Leptos CSR Dashboard — Dev Pipeline${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# --- Prerequisite Checks ---
check_cmd() {
    if ! command -v "$1" &>/dev/null; then
        echo -e "${RED}[ERROR]${NC} '$1' is not installed or not in PATH."
        case "$1" in
            trunk)
                echo "  Install: cargo install trunk"
                ;;
            bun)
                echo "  Install: curl -fsSL https://bun.sh/install | bash"
                ;;
            rustup)
                echo "  Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                ;;
            *)
                echo "  Please install '$1' and try again."
                ;;
        esac
        exit 1
    fi
}

check_cmd rustup
check_cmd cargo
check_cmd trunk
check_cmd bun

# Verify wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}[INFO]${NC} Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    echo -e "${GREEN}[OK]${NC} Target installed."
fi

# Verify Rust toolchain is nightly (required by leptos deps)
TOOLCHAIN=$(rustup default | head -1)
echo -e "${GREEN}[OK]${NC} Rust toolchain: ${TOOLCHAIN}"

# --- Port Detection ---
port_in_use() {
    local p=$1
    if command -v lsof &>/dev/null; then
        lsof -Pi :$p -sTCP:LISTEN -t &>/dev/null && return 0
    elif command -v ss &>/dev/null; then
        ss -tlnp "sport = :$p" 2>/dev/null | grep -q LISTEN && return 0
    elif [ -f /proc/net/tcp ]; then
        # /proc/net/tcp uses hex port numbers
        local hex
        hex=$(printf "%04X" "$p")
        awk '{print $2}' /proc/net/tcp 2>/dev/null | grep -qi ":${hex}$" && return 0
    else
        # Last resort: try binding a socket (race-condition prone, but best effort)
        (timeout 1 bash -c "exec 3>/dev/tcp/127.0.0.1/$p" 2>/dev/null) && return 0
    fi
    return 1
}

PORT=${PORT:-8080}
MAX_PORT=$((PORT + 100))
while port_in_use "$PORT"; do
    echo -e "${YELLOW}[WARN]${NC} Port $PORT is in use, trying $((PORT + 1))..."
    PORT=$((PORT + 1))
    if [ "$PORT" -ge "$MAX_PORT" ]; then
        echo -e "${RED}[ERROR]${NC} No free port found in range 8080-$((MAX_PORT - 1))"
        exit 1
    fi
done
echo -e "${GREEN}[OK]${NC} Port $PORT is available"

# --- Ensure output directories ---
mkdir -p public

# --- Start Tailwind CSS watcher ---
echo -e "${CYAN}[BUILD]${NC} Starting Tailwind CSS watcher..."
if [ ! -f style/tailwind.css ]; then
    echo -e "${YELLOW}[WARN]${NC} style/tailwind.css not found — creating empty file"
    mkdir -p style
    touch style/tailwind.css
fi

bun x tailwindcss -i ./style/tailwind.css -o ./public/tailwind.css --watch &
TAILWIND_PID=$!
echo -e "${GREEN}[OK]${NC} Tailwind CSS watcher started (PID: $TAILWIND_PID)"

# --- Cleanup handler ---
cleanup() {
    echo ""
    echo -e "${CYAN}[SHUTDOWN]${NC} Cleaning up..."
    kill $TAILWIND_PID 2>/dev/null || true
    echo -e "${GREEN}[OK]${NC} Tailwind CSS watcher stopped"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}
trap cleanup SIGINT SIGTERM

# --- Build & Serve ---
echo -e "${CYAN}[BUILD]${NC} Compiling and serving with Trunk..."
echo -e "${CYAN}[BUILD]${NC} Target: wasm32-unknown-unknown, Features: csr"
echo -e "${CYAN}[BUILD]${NC} Serving at: ${GREEN}http://127.0.0.1:$PORT${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

trunk serve --port "$PORT" || {
    echo -e "${RED}[ERROR]${NC} Trunk build or serve failed."
    echo "  Common causes:"
    echo "  - Rust compilation errors (check output above)"
    echo "  - Missing wasm32 target (run: rustup target add wasm32-unknown-unknown)"
    kill $TAILWIND_PID 2>/dev/null || true
    exit 1
}

cleanup
