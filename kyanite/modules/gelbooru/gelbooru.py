from lxml import html
import aiohttp
import hashlib

from kyanite.core.nodes.item import KyaniteItem


class HModule(object):
    def __init__(self, core):
        self.core = core
        self.id = 'gelbooru'
        self.name = 'Gelbooru Module'
        self.enabled = True
        self.api_base = 'https://gelbooru.com/index.php?page=dapi&s=post&q=index&tags='
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
            get_url = f'{api_url}&pid={page_num}'
            page_num += 1
            try:
                async with aiohttp.ClientSession() as session:
                    async with session.get(get_url) as data:
                        data = await data.read()
                        data = html.fromstring(data)
                        if len(data) == 0:
                            empty_page = True
                            print(f'Stopping at page {page_num}.')
                        else:
                            print(f'Found {len(data)} files on page {page_num}.')
                            self.core.total_counter += len(data)
                            for item in data:
                                url = item.attrib.get('file_url')
                                if url:
                                    ext = self.get_ext(url)
                                    crypt = hashlib.new('md5')
                                    crypt.update(url.encode('utf-8'))
                                    url_hash = crypt.hexdigest()
                                    item = {
                                        'md5': url_hash,
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
