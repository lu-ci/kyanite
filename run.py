import asyncio
from core.collector import KyaniteCollector


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    collector = KyaniteCollector()
    loop.run_until_complete(collector.run())
