#!/bin/bash
# Find an available port starting from the given port or 8080
PORT=${1:-8080}

while true; do
    # Check if port is in use
    if command -v lsof &>/dev/null; then
        if ! lsof -Pi :$PORT -sTCP:LISTEN -t &>/dev/null; then
            echo "$PORT"
            exit 0
        fi
    elif command -v ss &>/dev/null; then
        if ! ss -tlnp "sport = :$PORT" 2>/dev/null | grep -q LISTEN; then
            echo "$PORT"
            exit 0
        fi
    elif [ -f /proc/net/tcp ]; then
        hex=$(printf "%04X" "$PORT")
        if ! awk '{print $2}' /proc/net/tcp 2>/dev/null | grep -qi ":${hex}$"; then
            echo "$PORT"
            exit 0
        fi
    else
        # Fallback using /dev/tcp
        if ! (timeout 1 bash -c "exec 3>/dev/tcp/127.0.0.1/$PORT" 2>/dev/null); then
            echo "$PORT"
            exit 0
        fi
    fi
    PORT=$((PORT + 1))
    # Safety break to prevent infinite loop
    if [ "$PORT" -gt 65535 ]; then
        echo "Error: No available ports found" >&2
        exit 1
    fi
done
