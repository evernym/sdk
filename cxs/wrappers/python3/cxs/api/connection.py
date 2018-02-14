from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb
from cxs.api.cxs_stateful import CxsStateful

import json


class Connection(CxsStateful):

    def __init__(self, source_id: str):
        CxsStateful.__init__(self, source_id)

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Connection, self.handle))

    @staticmethod
    async def create(source_id: str):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_params = (c_source_id,)

        return await Connection._create(Connection,
                                        "cxs_connection_create",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def deserialize(data: dict):
        return await Connection._deserialize(Connection,
                                             "cxs_connection_deserialize",
                                             json.dumps(data),
                                             data.get('source_id'))

    async def connect(self, phone_number: Optional[str]) -> str:
        if not hasattr(Connection.connect, "cb"):
            self.logger.debug("cxs_connection_connect: Creating callback")
            Connection.connect.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        connection_data = {'connection_type': 'SMS', 'phone': phone_number} if phone_number \
            else {'connection_type': 'QR'}
        c_connection_data = c_char_p(json.dumps(connection_data).encode('utf-8'))
        invite_details = await do_call('cxs_connection_connect',
                                       c_connection_handle,
                                       c_connection_data,
                                       Connection.connect.cb)
        return invite_details

    async def serialize(self) -> dict:
        return await self._serialize(Connection, 'cxs_connection_serialize')

    async def update_state(self) -> int:
        return await self._update_state(Connection, 'cxs_connection_update_state')

    async def get_state(self) -> int:
        return await self._get_state(Connection, 'cxs_connection_get_state')

    def release(self) -> None:
        self._release(Connection, 'cxs_connection_release')

    async def invite_details(self, abbreviated: bool) -> dict:
        if not hasattr(Connection.invite_details, "cb"):
            self.logger.debug("cxs_connection_invite_details: Creating callback")
            Connection.invite_details.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        c_abbreviated = c_bool(abbreviated)

        details = await do_call('cxs_connection_invite_details',
                                c_connection_handle,
                                c_abbreviated,
                                Connection.invite_details.cb)

        return json.loads(details.decode())
