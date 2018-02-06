from ..common import do_call
from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb

import logging
import asyncio


class Connection:

    def __init__(self, source_id: str):
        self._source_id = source_id
        self._logger = logging.getLogger(__name__)
        self._connection_handle = 0

    def __del__(self):
        # destructor
        pass

    @staticmethod
    async def create(source_id: str):
        logger = logging.getLogger(__name__)
        connection = Connection(source_id)

        if not hasattr(Connection.create, "cb"):
            logger.debug("cxs_connection_create: Creating callback")
            Connection.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_source_id = c_char_p(source_id.encode('utf-8'))

        result = await do_call('cxs_connection_create',
                               c_source_id,
                               Connection.create.cb)

        connection._connection_handle = result
        return connection


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
