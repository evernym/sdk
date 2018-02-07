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

    @property
    def connection_handle(self):
        return self._connection_handle

    @connection_handle.setter
    def connection_handle(self, handle):
        self._connection_handle = handle

    @property
    def source_id(self):
        return self._source_id

    @source_id.setter
    def source_id(self, x):
        self._source_id = x

    @staticmethod
    async def create(source_id: str):
        connection = Connection(source_id)

        if not hasattr(Connection.create, "cb"):
            connection._logger.debug("cxs_connection_create: Creating callback")
            Connection.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_source_id = c_char_p(source_id.encode('utf-8'))

        result = await do_call('cxs_connection_create',
                               c_source_id,
                               Connection.create.cb)

        connection.connection_handle = result
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
