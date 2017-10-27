import asyncio


class HentaiCollector(object):
    def __init__(self):
        self.__version__ = 'a0.1.0'
        self.loop = asyncio.get_event_loop()
        self.collector = None
        self.tags = []
        self.files = []
        self.init_collector()

    def init_collector(self):
        from .collectors.yandere import YandereCollector
        collectors = {
            '0': YandereCollector()
        }
        print('----------------------------')
        for collector in collectors:
            print(f'\n{collector}: {collectors[collector].name}')
        print('\n----------------------------')
        choice = input('Input Option > ')
        if choice in collectors:
            self.collector = collectors[choice]
        else:
            exit('Invalid Choice.')

    def run(self):
        print('Input desired tag combination.')
        print('Leave blank if you are done.')
        ended = False
        while not ended:
            tag = input('> ')
            if tag:
                self.tags.append(tag)
            else:
                print('Tag choice ended.')
                ended = True
        self.files = self.loop.run_until_complete(self.collector.fill_urls(self.tags))
        from .mechanics.downloader import Downloader
        self.loop.run_until_complete(Downloader().download(self.files))
