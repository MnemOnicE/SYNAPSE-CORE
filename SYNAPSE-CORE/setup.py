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

from setuptools import setup, find_packages

setup(
    name="synapse",
    version="0.1.0",
    package_dir={"": "src"},
    packages=find_packages(where="src"),
    install_requires=[
        "redis==5.0.3",
        "python-dotenv==1.0.1",
    ],
)
