import json

import aiohttp

from kyanite.core.nodes.item import KyaniteItem


class HModule(object):
    def __init__(self, core):
        self.core = core
        self.id = 'e621'
        self.name = 'E621 Module'
        self.enabled = True
        self.api_base = 'https://e621.net/post/index.json?tags='
        self.collection = True
        self.tags = []
        self.headers = {'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:59.0) Gecko/20100101 Firefox/59.0'}

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
                    async with session.get(get_url, headers=self.headers) as data:
                        data = await data.text()
                        data = json.loads(data)
                        if len(data) == 0:
                            empty_page = True
                            print(f'Stopping at page {page_num}.')
                        else:
                            print(f'Found {len(data)} files on page {page_num}.')
                            self.core.total_counter += len(data)
                            for item in data:
                                url = item.get('file_url')
                                if url:
                                    ext = self.get_ext(url)
                                    item = {
                                        'md5': item.get('md5'),
                                        'file_ext': ext,
                                        'file_url': url
                                    }
                                    kya_item = KyaniteItem(self, self.tags, item)
                                    await self.core.queue.put(kya_item)
            except SyntaxError:
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
