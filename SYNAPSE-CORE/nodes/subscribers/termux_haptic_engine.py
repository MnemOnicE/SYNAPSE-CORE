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

import sys
import os

sys.path.append(
    os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
)

from core.broker_config import get_redis_client
from core.utils import PayloadManager
from core.logger import get_logger

logger = get_logger("termux_haptic_engine")


def execute_vibrate():
    """Simulate Termux vibration API call."""
    logger.info(f"📱 TRIGGERING HAPTIC MOTOR: termux-vibrate -d 1000 📱")


def main():
    try:
        redis_client = get_redis_client()
    except Exception:
        logger.error("Could not connect to Redis. Exiting.")
        return

    pubsub = redis_client.pubsub()
    pubsub.subscribe("synapse_bus")

    logger.info("Termux Haptic Engine initialized. Listening for 'alert' intents...")

    for message in pubsub.listen():
        if message["type"] == "message":
            raw_data = message["data"]
            payload = PayloadManager.parse_payload(raw_data)

            if not payload:
                continue

            if payload.get("action_intent") == "alert":
                execute_vibrate()


if __name__ == "__main__":
    main()
