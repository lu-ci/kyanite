import json
import asyncio
import aiohttp
from core.mechanics.item import KyaniteItem


class KonachanCollector(object):
    def __init__(self):
        self.name = 'Konachan Collector'
        self.location = 'kchan'
        self.api_base = 'https://konachan.com/post.json?limit=1000&tags='
        self.queue = asyncio.Queue()
        self.counter = 0
        self.done = 0
        self.skipped = 0
        self.failed = 0

    @staticmethod
    def get_ext(url):
        ext = url.lower().split('.')[-1]
        return ext

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
                            self.counter += len(data)
                            for item in data:
                                file_url = item['file_url']
                                if not file_url.startswith('http'):
                                    file_url = 'https:' + file_url
                                item.update({'file_url': file_url, 'file_ext': self.get_ext(file_url)})
                                kya_item = KyaniteItem(self, tags, item)
                                await self.queue.put(kya_item)
            except Exception:
                pass
