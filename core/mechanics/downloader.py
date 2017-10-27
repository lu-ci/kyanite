import os
import arrow
import aiohttp


class Downloader(object):
    def __init__(self):
        self.count = 0
        self.skipped = 0
        self.failed = 0
        self.success = 0
        self.current = 0

    async def download(self, files):
        cat = files[0]['cat']
        if not cat:
            cat = 'everything'
        if not os.path.exists('download'):
            os.makedirs('download')
        if not os.path.exists(f'download/{cat}'):
            os.makedirs(f'download/{cat}')
        for file in files:
            self.current += 1
            self.count = len(files)
            floc = f'download/{cat}/{file["fnam"]}'
            if not os.path.exists(floc):
                get_url = file['url']
                start_stamp = arrow.now().float_timestamp
                try:
                    async with aiohttp.ClientSession() as session:
                        async with session.get(get_url) as data:
                            data = await data.read()
                            with open(floc, 'wb') as file_out:
                                file_out.write(data)
                                self.success += 1
                                elapsed = arrow.now().float_timestamp - start_stamp
                                num_section = f'Number: {self.current}/{self.count}'
                                print(f'\rCompleted: {file["id"]} | {num_section} | Time: {round(elapsed, 2)}s')
                except Exception:
                    print(f'Failed: {file["id"]}')
                    self.failed += 1
            else:
                self.skipped += 1
                print(f'Skipping {file["id"]}...')
