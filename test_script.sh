# Run the Rust binary in the background
cargo run --manifest-path synapse/Cargo.toml &
RUST_PID=$!
sleep 1

# Run the python publisher
export PYTHONPATH=$(pwd)/SYNAPSE-CORE/src
python3 SYNAPSE-CORE/src/synapse/nodes/publishers/serial_listener.py &
PYTHON_PID=$!

# Let them run a bit
sleep 5

# Kill them
kill $RUST_PID
kill $PYTHON_PID
