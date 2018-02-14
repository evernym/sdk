from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb
from cxs.api.cxs_base import CxsBase

import logging
import json


class Connection(CxsBase):

    def __init__(self, source_id: str):
        CxsBase.__init__(self, source_id)
        self._logger = logging.getLogger(__name__)
        self._handle = 0
        self._state = 0

    def __del__(self):
        # destructor
        pass

    @property
    def state(self):
        return self._state

    @state.setter
    def state(self, x):
        self._state = x

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

        connection.handle = result
        connection._logger.debug("created connection object")
        return connection

    @staticmethod
    async def deserialize(data: dict):
        connection = await Connection._deserialize(Connection,
                                                   "cxs_connection_deserialize",
                                                   json.dumps(data),
                                                   data.get('source_id'))
        connection.state = data['state']
        return connection

    async def connect(self, phone_number: Optional[str]) -> None:
        if not hasattr(Connection.connect, "cb"):
            self._logger.debug("cxs_connection_connect: Creating callback")
            Connection.connect.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_connection_handle = c_uint32(self.handle)
        connection_data = {'connection_type': 'SMS', 'phone': phone_number} if phone_number \
            else {'connection_type': 'QR'}
        c_connection_data = c_char_p(json.dumps(connection_data).encode('utf-8'))

        await do_call('cxs_connection_connect',
                      c_connection_handle,
                      c_connection_data,
                      Connection.connect.cb)

    async def serialize(self) -> dict:
        return await self._serialize(Connection, 'cxs_connection_serialize')

    async def update_state(self) -> None:
        if not hasattr(Connection.update_state, "cb"):
            self._logger.debug("cxs_connection_update_state: Creating callback")
            Connection.update_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_connection_handle = c_uint32(self.handle)

        self.state = await do_call('cxs_connection_update_state',
                                   c_connection_handle,
                                   Connection.update_state.cb)

    async def release(self) -> None:
        await self._release(Connection, 'cxs_connection_release')

    async def invite_details(self, abbreviated: bool) -> dict:
        if not hasattr(Connection.invite_details, "cb"):
            self._logger.debug("cxs_connection_invite_details: Creating callback")
            Connection.invite_details.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        c_abbreviated = c_bool(abbreviated)

        details = await do_call('cxs_connection_invite_details',
                                c_connection_handle,
                                c_abbreviated,
                                Connection.invite_details.cb)

        return json.loads(details.decode())
