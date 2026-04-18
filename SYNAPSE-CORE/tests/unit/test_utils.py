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

import pytest
import json
from synapse.core.utils import PayloadManager

def test_build_payload():
    data = {"key": "value"}
    payload_str = PayloadManager.build_payload(
        source_node="test_node",
        event_category="test_category",
        action_intent="test_intent",
        data=data
    )

    payload = json.loads(payload_str)

    assert "timestamp" in payload
    assert payload["source_node"] == "test_node"
    assert payload["event_category"] == "test_category"
    assert payload["action_intent"] == "test_intent"
    assert payload["data"] == data

def test_parse_payload_valid():
    valid_payload_str = json.dumps({
        "timestamp": 1234567890,
        "source_node": "node",
        "event_category": "cat",
        "action_intent": "intent",
        "data": {}
    })

    parsed = PayloadManager.parse_payload(valid_payload_str)
    assert parsed is not None
    assert parsed["source_node"] == "node"

def test_parse_payload_invalid_schema():
    invalid_payload_str = json.dumps({
        "timestamp": 1234567890,
        "source_node": "node"
        # missing fields
    })

    parsed = PayloadManager.parse_payload(invalid_payload_str)
    assert parsed is None

def test_parse_payload_invalid_json():
    invalid_json = "{ invalid json"
    parsed = PayloadManager.parse_payload(invalid_json)
    assert parsed is None

def test_encode_decode_bytes():
    raw_bytes = b"test bytes"
    encoded = PayloadManager.encode_bytes(raw_bytes)
    assert isinstance(encoded, str)

    decoded = PayloadManager.decode_bytes(encoded)
    assert decoded == raw_bytes
