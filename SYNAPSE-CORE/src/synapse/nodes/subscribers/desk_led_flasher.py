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



from synapse.core.broker_config import get_redis_client
from synapse.core.utils import PayloadManager
from synapse.core.logger import get_logger

logger = get_logger("desk_led_flasher")


def execute_flash(severity: int):
    """Simulate flashing a desk LED based on severity."""
    logger.info(f"⚡ FLASHING DESK LED ⚡ (Severity: {severity})")


def execute_binary_parse(encoded_data: str):
    """Simulate decoding a binary payload."""
    try:
        raw_bytes = PayloadManager.decode_bytes(encoded_data)
        logger.info(f"🔍 DECODED RAW BYTES from sensor: {raw_bytes}")
    except Exception as e:
        logger.error(f"Failed to decode binary data: {e}")


def main():
    try:
        redis_client = get_redis_client()
    except Exception:
        logger.error("Could not connect to Redis. Exiting.")
        return

    pubsub = redis_client.pubsub()
    pubsub.subscribe("synapse_bus")

    logger.info(
        "Desk LED Flasher initialized. Listening for 'alert' intents and 'hardware_sensor_read' events..."
    )

    for message in pubsub.listen():
        if message["type"] == "message":
            raw_data = message["data"]
            payload = PayloadManager.parse_payload(raw_data)

            if not payload:
                logger.warning(f"Received malformed payload on bus: {raw_data}")
                continue

            # React to an alert
            if payload.get("action_intent") == "alert":
                severity = payload.get("data", {}).get("severity", 1)
                execute_flash(severity)

            # React to a binary sensor read (demonstrating Base64 decoding)
            if payload.get("event_category") == "hardware_sensor_read":
                encoded = payload.get("data", {}).get("encoded_payload")
                if encoded:
                    execute_binary_parse(encoded)


if __name__ == "__main__":
    main()
