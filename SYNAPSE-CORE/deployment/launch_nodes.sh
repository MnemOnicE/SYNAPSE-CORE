#!/bin/bash

# Navigate to the root of SYNAPSE-CORE
cd "$(dirname "$0")/.."

echo "Starting Project SYNAPSE..."

# Check for .env file
if [ ! -f .env ]; then
    echo "Error: .env file not found. Please ensure it exists."
    # Using return instead of exit for safe shell environments
    return 1
fi

# Ensure python environment exists
if [ ! -d "venv" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv venv
    source venv/bin/activate
    echo "Installing dependencies..."
    pip install -r requirements.txt
else
    source venv/bin/activate
fi

# Ensure Redis is running via Docker Compose
if command -v docker-compose &> /dev/null; then
    echo "Starting Redis broker via Docker Compose..."
    docker-compose -f deployment/docker-compose.yml up -d
else
    echo "Warning: docker-compose not found. Make sure your Redis broker is running manually."
fi

# Give Redis a moment to start
sleep 2

echo "Launching Synapse Nodes..."

# Launch Hybrid Nodes (The Brain)
python nodes/hybrids/logic_router.py &
LOGIC_PID=$!

# Launch Subscriber Nodes (Motor/UI)
python nodes/subscribers/desk_led_flasher.py &
LED_PID=$!
python nodes/subscribers/termux_haptic_engine.py &
HAPTIC_PID=$!

# Launch Publisher Nodes (Sensors)
python nodes/publishers/angler_watchdog.py &
ANGLER_PID=$!
python nodes/publishers/serial_listener.py &
SERIAL_PID=$!

echo "All nodes launched."
echo "Press Ctrl+C to terminate all nodes."

# Trap SIGINT to kill all child processes gracefully
trap "echo 'Terminating all nodes...'; kill $LOGIC_PID $LED_PID $HAPTIC_PID $ANGLER_PID $SERIAL_PID" SIGINT

# Wait for all background processes
wait
