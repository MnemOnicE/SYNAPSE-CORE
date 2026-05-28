"""
---
aliases: [Project SYNAPSE, The Living Blade, Axion Core]
tags:
  - #state/crystallized
  - #process/as-within-so-throughout
  - #architecture/distributed
parent_concept: "Cybernetic Frameworks"
child_concepts: ["Watchdog Node", "Transducer Node", "Observer Node", "Message Bus"]
sibling_concepts: ["Coding Squad", "KnoNav", "Credon Protocol"]
---
"""

import timeit
import json


def run_benchmark():
    setup_code = """
import json
from synapse.core.utils import PayloadManager

valid_payload_str = json.dumps({
    "timestamp": 1234567890,
    "source_node": "node",
    "event_category": "cat",
    "action_intent": "intent",
    "data": {}
})
"""

    stmt = "PayloadManager.parse_payload(valid_payload_str)"

    # Run a substantial number of iterations to get a stable measurement
    number = 100000

    time_taken = timeit.timeit(stmt, setup=setup_code, number=number)

    print(f"Benchmark: PayloadManager.parse_payload")
    print(f"Iterations: {number}")
    print(f"Total time: {time_taken:.6f} seconds")
    print(f"Time per parse: {(time_taken / number) * 1e6:.6f} microseconds")


if __name__ == "__main__":
    run_benchmark()
