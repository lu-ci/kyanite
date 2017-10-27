import os
import aiohttp


class KyaniteItem(object):
    def __init__(self, downloader, tags, item_data):
        self.downloader = downloader
        self.tags = sorted(tags) or ['everything']
        self.item_data = item_data
        self.dl_tag = self.downloader.location
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
        self.downloader.done += 1
        if not self.does_exist():
            try:
                async with aiohttp.ClientSession() as session:
                    async with session.get(self.url) as data:
                        data = await data.read()
                        with open(self.output, 'wb') as file_out:
                            file_out.write(data)
                print(f'Complete: {self.id} | {self.downloader.done}/{self.downloader.counter}')
            except Exception:
                self.downloader.failed += 1
                print(f'Failure: {self.id} | No. {self.downloader.failed}')
        else:
            self.downloader.skipped += 1
            print(f'Skipped: {self.id} | No. {self.downloader.skipped}')
