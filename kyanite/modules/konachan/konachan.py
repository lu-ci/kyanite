import json

import aiohttp

from kyanite.core.nodes.item import KyaniteItem


class HModule(object):
    def __init__(self, core):
        self.core = core
        self.id = 'konachan'
        self.name = 'Konachan Module'
        self.enabled = True
        self.api_base = 'https://konachan.com/post.json?limit=1000&tags='
        self.collection = True
        self.tags = []

    @staticmethod
    def get_ext(url):
        ext = url.lower().split('.')[-1]
        return ext

    async def collect(self):
        print(f'Running {self.id.title()} Collector...')
        api_url = f'{self.api_base}{"+".join(self.tags)}'
        tries = 0
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
                    self.core.total_counter += len(data)
                    for item in data:
                        file_url = item['file_url']
                        if not file_url.startswith('http'):
                            file_url = 'https:' + file_url
                        item.update({'file_url': file_url, 'file_ext': self.get_ext(file_url)})
                        kya_item = KyaniteItem(self, self.tags, item)
                        await self.core.queue.put(kya_item)
            except Exception:
                print('Failed to grab one of the pages.')
                if tries >= 3:
                    empty_page = True
                else:
                    tries += 1

    async def execute(self, tags=None):
        if not tags:
            self.tags = self.core.tagger()
        else:
            self.tags = tags
        self.tags = sorted(self.tags)
        await self.collect()
