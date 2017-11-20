import importlib
import os


class ModuleLoader(object):
    def __init__(self, core):
        self.core = core
        self.base = 'kyanite/modules'
        self.modules = []
        self.load_all_modules()

    def load_all_modules(self):
        for root, dirs, files in os.walk(self.base):
            for file in files:
                if file.endswith('.py'):
                    filename = file.split('.')[0]
                    module_location = os.path.join(root, f'{filename}')
                    module_location = module_location.replace('/', '.')
                    module_location = module_location.replace('\\', '.')
                    module_class = importlib.import_module(module_location).HModule(self.core)
                    self.modules.append(module_class)
