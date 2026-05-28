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

import base64
import json
import time
from typing import Any, Dict, Optional


class PayloadManager:
    """Handles parsing, building, and verifying the standardized Synapse JSON schema."""

    REQUIRED_KEYS = frozenset({
        "timestamp",
        "source_node",
        "event_category",
        "action_intent",
        "data",
    })

    @staticmethod
    def build_payload(
        source_node: str, event_category: str, action_intent: str, data: Dict[str, Any]
    ) -> str:
        """
        Builds a strictly compliant JSON payload.
        Auto-generates the timestamp.
        """
        payload = {
            "timestamp": int(time.time()),
            "source_node": source_node,
            "event_category": event_category,
            "action_intent": action_intent,
            "data": data,
        }
        return json.dumps(payload)

    @staticmethod
    def parse_payload(raw_message: str) -> Optional[Dict[str, Any]]:
        """
        Parses a raw message string into a dict, verifying the schema fields.
        Returns None if parsing fails or schema is violated.
        """
        try:
            payload = json.loads(raw_message)

            if not isinstance(payload, dict) or not PayloadManager.REQUIRED_KEYS.issubset(payload.keys()):
                raise ValueError("Missing required keys in payload schema.")

            return payload
        except (json.JSONDecodeError, ValueError):
            # We don't log directly here to avoid circular imports, caller handles it.
            return None

    @staticmethod
    def encode_bytes(data: bytes) -> str:
        """Encodes raw bytes to base64 string for safe JSON serialization."""
        return base64.b64encode(data).decode("utf-8")

    @staticmethod
    def decode_bytes(encoded_str: str) -> bytes:
        """Decodes base64 string back to raw bytes."""
        try:
            return base64.b64decode(encoded_str)
        except Exception:
            return b""
