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

logger = get_logger("logic_router")


def main():
    try:
        redis_client = get_redis_client()
    except Exception:
        logger.error("Could not connect to Redis. Exiting.")
        return

    pubsub = redis_client.pubsub()
    pubsub.subscribe("synapse_bus")

    logger.info(
        "Logic Router (The Brain) initialized. Listening for events to process..."
    )

    # Simulated state
    is_compiling_knonav = True

    for message in pubsub.listen():
        if message["type"] == "message":
            raw_data = message["data"]
            payload = PayloadManager.parse_payload(raw_data)

            if not payload:
                continue

            # Brain Logic: Listen to sensory events, decide what to do
            if payload.get("event_category") == "hardware_button_press":
                button_id = payload.get("data", {}).get("button_id")

                logger.info(f"Brain received button press: {button_id}")

                if button_id == "panic_button_1" and is_compiling_knonav:
                    logger.info("Brain determining action: Abort build required!")

                    # Publish a motor command back to the bus
                    command_payload = PayloadManager.build_payload(
                        source_node="logic_router",
                        event_category="system_command",
                        action_intent="abort_build",
                        data={"reason": "Panic button pressed during compile"},
                    )
                    redis_client.publish("synapse_bus", command_payload)
                    logger.info(f"Brain published command: {command_payload}")


if __name__ == "__main__":
    main()
