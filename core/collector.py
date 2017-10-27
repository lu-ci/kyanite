class KyaniteCollector(object):
    def __init__(self):
        self.__version__ = 'a0.1.0'
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

    async def run(self):
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
        await self.collector.fill_urls(self.tags)
        while not self.collector.queue.empty():
            item = await self.collector.queue.get()
            await item.download()
