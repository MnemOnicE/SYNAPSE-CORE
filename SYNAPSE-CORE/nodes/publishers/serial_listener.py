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

import time
import sys
import os

# Add the root directory to path to allow importing 'core'
sys.path.append(
    os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
)

from core.broker_config import get_redis_client
from core.utils import PayloadManager
from core.logger import get_logger

logger = get_logger("serial_listener")


def main():
    try:
        redis_client = get_redis_client()
    except Exception:
        logger.error("Could not connect to Redis. Exiting.")
        return

    logger.info(
        "Serial Listener initialized. Simulating serial port reads with binary payloads..."
    )

    # Simulate an observation loop
    for i in range(3):
        time.sleep(7)

        # Simulate reading a raw byte sequence from a hardware sensor (e.g. \x01\x02\x03)
        raw_sensor_bytes = b"\x01\x02\x03\x04\xff\x00"
        encoded_data = PayloadManager.encode_bytes(raw_sensor_bytes)

        payload = PayloadManager.build_payload(
            source_node="serial_listener",
            event_category="hardware_sensor_read",
            action_intent="none",
            data={
                "sensor_id": "temp_sensor_bus_A",
                "encoded_payload": encoded_data,
                "note": "Encoded base64 binary payload",
            },
        )

        redis_client.publish("synapse_bus", payload)
        logger.info(f"Published hardware event to bus: {payload}")


if __name__ == "__main__":
    main()
