import os
import aiohttp


class KyaniteItem(object):
    def __init__(self, downloader, tags, item_data):
        self.downloader = downloader
        self.tags = sorted(tags) or ['everything']
        self.id = item_data['md5']
        self.ext = item_data['file_ext']
        self.name = f'{self.id}.{self.ext}'
        self.category = '_'.join(tags)
        self.url = item_data['file_url']
        self.folder = f'download/{self.downloader}/{self.category}'
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
                async with aiohttp.ClientSession() as session:
                    async with session.get(self.url) as data:
                        data = await data.read()
                        with open(self.output, 'wb') as file_out:
                            file_out.write(data)
                print(f'Complete: {self.id}')
            except Exception:
                print(f'Failure: {self.id}')
        else:
            print(f'Skipped: {self.id}')
