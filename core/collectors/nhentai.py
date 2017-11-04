import asyncio

import aiohttp
from lxml import html

from core.mechanics.item import KyaniteItem


class NHentaiCollector(object):
    def __init__(self):
        self.name = 'nHentai Collector'
        self.location = 'nh'
        self.gallery_base = 'https://nhentai.net/g'
        self.queue = asyncio.Queue()
        self.counter = 0
        self.done = 0
        self.skipped = 0
        self.failed = 0

    async def fill_urls(self, tags):
        if tags[0].startswith('http'):
            gallery_id = tags[0].split('/')
            gallery_id = list(filter(lambda a: a != '', gallery_id))[-1]
        else:
            gallery_id = tags[0]
        print('Running Collector...')
        api_url = f'{self.gallery_base}/{gallery_id}'
        async with aiohttp.ClientSession() as session:
            async with session.get(api_url) as data:
                data = await data.text()
                data = html.fromstring(data)
        page_num = len(data.cssselect('#thumbnail-container')[0].cssselect('.gallerythumb'))
        print(f'Found {page_num} Images...')
        for x in range(1, page_num + 1):
            get_url = f'{api_url}/{x}/'
            try:
                async with aiohttp.ClientSession() as session:
                    async with session.get(get_url) as data:
                        data = await data.text()
                        data = html.fromstring(data)
                        image = data.cssselect('.fit-horizontal')[0].attrib.get('src')
                        self.counter += len(data)
                        item = {
                            'md5': f'{gallery_id}_{x}',
                            'file_ext': image.split('.')[-1],
                            'file_url': image
                        }
                        kya_item = KyaniteItem(self, [gallery_id], item)
                        await self.queue.put(kya_item)
            except Exception:
                pass
