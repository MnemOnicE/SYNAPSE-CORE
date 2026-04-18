#!/bin/bash

# Navigate to the root of SYNAPSE-CORE
cd "$(dirname "$0")/.."

echo "Starting Project SYNAPSE..."

# Check for .env file
if [ ! -f .env ]; then
    echo "Warning: .env file not found. Falling back to defaults."
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

# Export PYTHONPATH to allow absolute imports within the 'src' directory
export PYTHONPATH=src

# Launch Hybrid Nodes (The Brain)
python -m synapse.nodes.hybrids.logic_router &
LOGIC_PID=$!

# Launch Subscriber Nodes (Motor/UI)
python -m synapse.nodes.subscribers.desk_led_flasher &
LED_PID=$!
python -m synapse.nodes.subscribers.termux_haptic_engine &
HAPTIC_PID=$!

# Launch Publisher Nodes (Sensors)
python -m synapse.nodes.publishers.angler_watchdog &
ANGLER_PID=$!
python -m synapse.nodes.publishers.serial_listener &
SERIAL_PID=$!

echo "All nodes launched."
echo "Press Ctrl+C to terminate all nodes."

# Trap SIGINT to kill all child processes gracefully
trap "echo 'Terminating all nodes...'; kill \$(jobs -p) 2>/dev/null" SIGINT

# Wait for all background processes
wait
