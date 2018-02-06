from ..cxs import do_call
from typing import Optional
from ctypes import *

import logging
import asyncio


class Connection:

    def __init__(self, source_id: str):
        self._source_id = source_id
        self._logger = logging.getLogger(__name__)

    def __del__(self):
        # destructor
        pass

    @staticmethod
    async def create(source_id: str):
        connection = Connection(source_id)
        if not hasattr(create, 'cb'):
            create.cb = create_cb(CFUNCTYPE(None, c_uint32, ))

    @staticmethod
    async def deserialize(source_id: str):
        pass

    async def connect(self, source_id: str):
        pass

    async def serialize(self, source_id: str):
        pass

    async def update_state(self, source_id: str):
        pass

    @staticmethod
    def random_test(self):
        print('test')

async def main():
    await Connection.create('1')

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    # time.sleep(1)  # FIXME waiting for libindy thread complete