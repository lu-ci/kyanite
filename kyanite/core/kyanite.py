import asyncio

from kyanite.core.nodes.modules import ModuleLoader


class Kyanite(object):
    def __init__(self):
        self.__version__ = '0.3.0'
        self.loop = asyncio.get_event_loop()
        self.queue = asyncio.Queue()
        self.modules = []
        self.ongoing = []
        self.init_modules()
        self.total_counter = 0
        self.complete_counter = 0

    def init_modules(self):
        print('Loading modules...')
        loader = ModuleLoader(self)
        self.modules = loader.modules
        print(f'Loaded {len(self.modules)} Modules.')

    @staticmethod
    def split():
        print('------------------------------')

    def selector(self):
        ended = False
        mode_out = None
        module_range = len(self.modules)
        while not ended:
            mode_input = input('> ')
            try:
                mode_index = int(mode_input)
            except ValueError:
                mode_index = None
            if mode_input is not None:
                if mode_index in range(0, module_range) or mode_index in [99, 999]:
                    mode_out = mode_index
                    ended = True
                else:
                    print('That\'s not a valid option.')
            else:
                print('You did not enter a number.')
        return mode_out

    @staticmethod
    def tagger():
        ended = False
        tags = []
        print('Please input your desired tags.')
        print('To end the tag input just submit a blank line.')
        while not ended:
            tag_input = input('> ')
            if tag_input:
                tag_addition = tag_input.split(' ')
                tags += tag_addition
            else:
                if tags:
                    ended = True
                else:
                    print('Can\'t have an empty tag list.')
        return tags

    async def bulk_collector(self):
        tags = self.tagger()
        for kya_module in self.modules:
            if kya_module.collection:
                await kya_module.execute(tags=tags)

    def check_ongoing(self):
        for task in self.ongoing:
            if task.done():
                self.ongoing.remove(task)

    async def download(self):
        while not self.queue.empty() or self.ongoing:
            self.check_ongoing()
            if len(self.ongoing) < 10:
                self.complete_counter += 1
                item = await self.queue.get()
                task = self.loop.create_task(item.download())
                self.ongoing.append(task)
            else:
                await asyncio.sleep(1)

    def run(self):
        self.split()
        print('Please select a module.')
        self.split()
        loop_index = 0
        for kya_module in self.modules:
            print(f'{loop_index}: {kya_module.name}')
        print(f'99: All Modules')
        print(f'999: Exit')
        self.split()
        mode = self.selector()
        if mode == 99:
            self.loop.run_until_complete(self.bulk_collector())
            self.loop.run_until_complete(self.download())
        elif mode == 999:
            exit(0)
        else:
            self.loop.run_until_complete(self.modules[mode].execute())
            self.loop.run_until_complete(self.download())
