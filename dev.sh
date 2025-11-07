#!/bin/bash
# Development helper script for 12factor.net
# Manages the Rust server with auto-restart and clear error messages

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Directories
PROJECT_ROOT="$(cd "$(dirname "$0")" && pwd)"
RUST_DIR="$PROJECT_ROOT/rust_prototype"

# Port - 12012 is a fun pun for 12factor!
PORT=12012

# Function to kill process on port
kill_port() {
    local port=$1
    local pid=$(lsof -ti:$port 2>/dev/null)
    if [ ! -z "$pid" ]; then
        echo -e "${YELLOW}Killing process on port $port (PID: $pid)${NC}"
        kill $pid 2>/dev/null || true
        sleep 1
    fi
}

# Function to start server
start_server() {
    echo -e "${GREEN}Starting server on port $PORT...${NC}"
    kill_port $PORT
    
    # Build if needed
    if [ ! -f "$RUST_DIR/target/release/twelve_factor" ] || [ "$1" == "rebuild" ]; then
        echo -e "${YELLOW}Building Rust server...${NC}"
        cd "$RUST_DIR"
        cargo build --release 2>&1 | tee rust_build.log
        if [ ${PIPESTATUS[0]} -ne 0 ]; then
            echo -e "${RED}Build failed! Check rust_build.log${NC}"
            return 1
        fi
    fi
    
    # Start server
    "$RUST_DIR/target/release/twelve_factor" > "$RUST_DIR/server.log" 2>&1 &
    echo $! > "$RUST_DIR/server.pid"
    echo -e "${GREEN}Server started (PID: $(cat "$RUST_DIR/server.pid"))${NC}"
}

# Function to check server health
check_health() {
    if curl -s -f -o /dev/null "http://localhost:$PORT/"; then
        echo -e "${GREEN}✓ Server is healthy${NC}"
        return 0
    else
        echo -e "${RED}✗ Server is not responding${NC}"
        return 1
    fi
}

# Function to show logs
show_logs() {
    echo -e "${YELLOW}=== Recent server logs ===${NC}"
    tail -n 20 "$RUST_DIR/server.log" 2>/dev/null || echo "No logs yet"
}

# Function to open browser
open_browser() {
    echo -e "${YELLOW}Opening browser...${NC}"
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:$PORT" &
    elif command -v open &> /dev/null; then
        open "http://localhost:$PORT" &
    else
        echo -e "${YELLOW}Please open: http://localhost:$PORT${NC}"
    fi
}

# Main command handling
case "${1:-start}" in
    start)
        echo -e "${GREEN}Starting development server...${NC}"
        start_server
        sleep 3
        check_health
        open_browser
        ;;
    
    stop)
        echo -e "${YELLOW}Stopping server...${NC}"
        kill_port $PORT
        if [ -f "$RUST_DIR/server.pid" ]; then
            rm -f "$RUST_DIR/server.pid"
        fi
        ;;
    
    restart)
        $0 stop
        sleep 1
        $0 start
        ;;
    
    rebuild)
        echo -e "${YELLOW}Rebuilding and restarting server...${NC}"
        kill_port $PORT
        start_server rebuild
        sleep 2
        check_health
        ;;
    
    status)
        check_health
        if [ -f "$RUST_DIR/server.pid" ]; then
            echo -e "${GREEN}PID: $(cat "$RUST_DIR/server.pid")${NC}"
        fi
        ;;
    
    logs)
        show_logs
        ;;
    
    watch)
        echo -e "${GREEN}Watching for crashes...${NC}"
        echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
        while true; do
            if ! check_health &>/dev/null; then
                echo -e "${RED}Server crashed, restarting...${NC}"
                start_server
            fi
            sleep 5
        done
        ;;
    
    *)
        echo "Usage: $0 {start|stop|restart|rebuild|status|logs|watch}"
        echo ""
        echo "Commands:"
        echo "  start    - Start the server"
        echo "  stop     - Stop the server"
        echo "  restart  - Restart the server"
        echo "  rebuild  - Rebuild and restart server"
        echo "  status   - Check server health"
        echo "  logs     - Show recent server logs"
        echo "  watch    - Watch for crashes and auto-restart"
        exit 1
        ;;
esac