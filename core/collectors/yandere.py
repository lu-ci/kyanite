import json
import aiohttp


class YandereCollector(object):
    def __init__(self):
        self.name = 'Yande.re Collector'
        self.link_list = []
        self.api_base = 'https://yande.re/post.json?limit=1000&tags='

    async def fill_urls(self, tags):
        print('Running Collector...')
        api_url = f'{self.api_base}{"+".join(tags)}'
        page_num = 0
        empty_page = False
        while not empty_page:
            page_num += 1
            get_url = f'{api_url}&page={page_num}'
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
                            item_data = {
                                'id': item['md5'],
                                'ext': item['file_ext'],
                                'fnam': f'{item["md5"]}.{item["file_ext"]}',
                                'cat': "_".join(sorted(tags)),
                                'url': item['file_url']
                            }
                            self.link_list.append(item_data)
        return self.link_list
