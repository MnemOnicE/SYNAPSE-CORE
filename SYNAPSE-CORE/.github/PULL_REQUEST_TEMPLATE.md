## Description
Provide a brief overview of the changes made in this PR. What new capability or bug fix does this introduce to Project SYNAPSE?

## Node Archetype
_If adding a new node, what is it? (Delete the ones that don't apply)_
- [ ] Publisher (Sensory Node)
- [ ] Subscriber (Motor Node)
- [ ] Hybrid (Logic/Brain Node)
- [ ] Core Infrastructure Change

## Payload Schema Additions
_If your node introduces a new `event_category` or `action_intent`, document it here:_
- **New Event Category:** `none`
- **New Action Intent:** `none`

## Checklist
- [ ] My code follows the core architectural guidelines (no spaghetti dependencies).
- [ ] I have used `PayloadManager` to build/parse payloads.
- [ ] Raw byte payloads use the Base64 utility functions.
- [ ] My script includes the required Obsidian YAML frontmatter block.
- [ ] I have tested this locally using `launch_nodes.sh`.
