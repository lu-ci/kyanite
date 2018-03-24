import os

import aiohttp


class KyaniteItem(object):
    def __init__(self, downloader, tags, item_data):
        self.downloader = downloader
        self.tags = sorted(tags) or ['everything']
        self.item_data = item_data
        self.dl_tag = self.downloader.id
        self.id = self.item_data['md5']
        self.ext = self.item_data['file_ext']
        self.name = f'{self.id}.{self.ext}'
        self.category = '_'.join(tags)
        self.url = self.item_data['file_url']
        self.folder = f'download/{self.dl_tag}/{self.category}'
        self.output = f'{self.folder}/{self.name}'
        self.check_folders()

    def check_folders(self):
        if not os.path.exists(self.folder):
            os.makedirs(self.folder)

    def does_exist(self):
        if os.path.exists(self.output):
            exists = True
        else:
            exists = False
        return exists

    async def download(self):
        if not self.does_exist():
            try:
                self.downloader.core.complete_counter += 1
                async with aiohttp.ClientSession() as session:
                    async with session.get(self.url) as data:
                        data = await data.read()
                        with open(self.output, 'wb') as file_out:
                            file_out.write(data)
                total = self.downloader.core.total_counter
                done = self.downloader.core.complete_counter
                print(f'Complete: {self.id} | {done}/{total}')
            except Exception:
                print(f'Failure: {self.id}')
        else:
            print(f'Skipped: {self.id}')
