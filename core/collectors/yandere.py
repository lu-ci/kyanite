import json
import asyncio
import aiohttp
from core.mechanics.item import KyaniteItem


class YandereCollector(object):
    def __init__(self):
        self.name = 'Yande.re Collector'
        self.location = 'yre'
        self.api_base = 'https://yande.re/post.json?limit=1000&tags='
        self.queue = asyncio.Queue()

    async def fill_urls(self, tags):
        print('Running Collector...')
        api_url = f'{self.api_base}{"+".join(tags)}'
        page_num = 0
        empty_page = False
        while not empty_page:
            page_num += 1
            get_url = f'{api_url}&page={page_num}'
            try:
                async with aiohttp.ClientSession() as session:
                    async with session.get(get_url) as data:
                        data = await data.read()
                        data = json.loads(data)
                        if not data:
                            empty_page = True
                            print(f'Stopping at page {page_num}.')
                        else:
                            print(f'Found {len(data)} files on page {page_num}.')
                            for item in data:
                                kya_item = KyaniteItem(self.location, tags, item)
                                await self.queue.put(kya_item)
            except Exception:
                pass
