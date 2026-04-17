import os
import redis
from dotenv import load_dotenv

from .logger import get_logger

logger = get_logger("broker_config")


def get_redis_client() -> redis.Redis:
    """
    Establishes and returns an authenticated Redis client.
    Pulls credentials from the environment variables (.env).
    """
    # Load .env file from the root directory of the project
    env_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), ".env")
    load_dotenv(dotenv_path=env_path)

    host = os.getenv("REDIS_HOST", "localhost")
    try:
        port = int(os.getenv("REDIS_PORT", 6379))
    except (ValueError, TypeError):
        logger.error("Invalid REDIS_PORT value. Defaulting to 6379.")
        port = 6379

    password = os.getenv("REDIS_PASSWORD")

    if not password:
        logger.warning(
            "REDIS_PASSWORD is not set! Connection may fail if broker requires auth."
        )

    client = redis.Redis(
        host=host,
        port=port,
        password=password,
        decode_responses=True,  # Automatically decode byte responses to strings
    )

    # Test connection
    try:
        client.ping()
        logger.info(f"Successfully connected to Redis broker at {host}:{port}")
    except redis.ConnectionError as e:
        logger.error(f"Failed to connect to Redis broker: {e}")
        raise

    return client
