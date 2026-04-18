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


from synapse.core.broker_config import get_redis_client
from synapse.core.utils import PayloadManager
from synapse.core.logger import get_logger

logger = get_logger("angler_watchdog")


def main():
    try:
        redis_client = get_redis_client()
    except Exception:
        logger.error("Could not connect to Redis. Exiting.")
        return

    logger.info("Angler Watchdog initialized. Simulating sensory observations...")

    # Simulate an observation loop
    for i in range(3):
        time.sleep(5)  # Wait 5 seconds between observations

        # Simulate a security event detection
        payload = PayloadManager.build_payload(
            source_node="ubuntu_auth_watchdog",
            event_category="security",
            action_intent="alert",
            data={
                "ip_address": "192.168.1.45",
                "severity": 8,
                "note": f"Simulated detection #{i + 1}",
            },
        )

        # Publish to the 'synapse_bus' channel
        redis_client.publish("synapse_bus", payload)
        logger.info(f"Published alert to bus: {payload}")


if __name__ == "__main__":
    main()
