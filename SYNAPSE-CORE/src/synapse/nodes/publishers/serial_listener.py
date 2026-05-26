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
import socket
import json
import os

from synapse.core.utils import PayloadManager
from synapse.core.logger import get_logger

logger = get_logger("serial_listener")


def main():
    logger.info(
        "Serial Listener initialized. Simulating serial port reads with binary payloads..."
    )

    socket_path = os.path.expanduser("~/.synapse/ingress.sock")

    # Simulate an observation loop
    for i in range(3):
        time.sleep(1)

        # Simulate reading a raw byte sequence from a hardware sensor (e.g. \x01\x02\x03)
        raw_sensor_bytes = b"\x01\x02\x03\x04\xff\x00"
        encoded_data = PayloadManager.encode_bytes(raw_sensor_bytes)

        payload_dict = {
            "source_node": "serial_listener",
            "event_category": "hardware_sensor_read",
            "action_intent": "none",
            "data": {
                "sensor_id": "temp_sensor_bus_A",
                "encoded_payload": encoded_data,
                "note": "Encoded base64 binary payload",
            },
        }

        try:
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
                client.connect(socket_path)
                serialized = json.dumps(payload_dict)
                client.sendall(serialized.encode('utf-8'))
            logger.info(f"Published hardware event to UDS bus: {payload_dict['event_category']}")

        except Exception as e:
            logger.error(f"Failed to publish to UDS bus: {e}")


if __name__ == "__main__":
    main()
