from cxs.common import do_call, create_cb
from ctypes import *

import logging
import json


class CxsBase:

    def __init__(self, source_id: str):
        self._source_id = source_id
        self._logger = logging.getLogger(__name__)
        self._handle = None

    @property
    def handle(self):
        return self._handle

    @handle.setter
    def handle(self, handle):
        self._handle = handle

    @property
    def source_id(self):
        return self._source_id

    @source_id.setter
    def source_id(self, x):
        self._source_id = x

    @property
    def logger(self):
        return self._logger

    @staticmethod
    async def _deserialize(cls, fn: str, data: str, *args):
        obj = cls(*args)

        if not hasattr(cls.deserialize, "cb"):
            obj.logger.debug("{}: Creating callback".format(fn))
            cls.deserialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_data = c_char_p(data.encode('utf-8'))

        result = await do_call(fn,
                               c_data,
                               cls.deserialize.cb)

        obj.handle = result
        obj.logger.debug("created {} object".format(cls))
        return obj

    async def _serialize(self, cls, fn: str) -> dict:
        if not hasattr(cls.serialize, "cb"):
            self.logger.debug("{}: Creating callback".format(fn))
            cls.serialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_handle = c_uint32(self.handle)

        data = await do_call(fn,
                             c_handle,
                             cls.serialize.cb)

        self.logger.debug("serialized {} object".format(cls))
        return json.loads(data.decode())

    async def _release(self, cls, fn: str):
        if not hasattr(cls.release, "cb"):
            self.logger.debug("{}: Creating callback".format(fn))
            cls.release.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        await do_call(fn,
                      c_handle,
                      cls.release.cb)
