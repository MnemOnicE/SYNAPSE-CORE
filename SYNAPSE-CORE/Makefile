.PHONY: setup test run lint clean

setup:
	python3 -m venv venv
	./venv/bin/pip install -r requirements.txt
	./venv/bin/pip install -e .

test:
	PYTHONPATH=src pytest tests/

run:
	./deployment/launch_nodes.sh

lint:
	flake8 src/ tests/

clean:
	rm -rf venv/
	rm -rf src/synapse.egg-info/
	find . -type d -name "__pycache__" -exec rm -rf {} +
	find . -type d -name ".pytest_cache" -exec rm -rf {} +
