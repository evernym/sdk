from ctypes import *
from cxs.common import do_call, create_cb
from cxs.error import CxsError, ErrorCode
from cxs.api.cxs_base import CxsBase

import json


class Schema(CxsBase):

    def __init__(self, source_id: str, name: str, attr_names: list):
        CxsBase.__init__(self, source_id)
        self._source_id = source_id
        self._attrs = attr_names
        self._name = name

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Schema, self.handle))

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, x):
        self._name = x

    @property
    def attrs(self):
        return self._attrs

    @attrs.setter
    def attrs(self, x):
        self._attrs = x

    @staticmethod
    async def create(source_id: str, name: str, attr_names: list):
        constructor_params = (source_id, name, attr_names)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_schema_data = c_char_p(json.dumps(attr_names).encode('utf-8'))
        c_params = (c_source_id, c_name, c_schema_data)

        return await Schema._create(Schema,
                                    "cxs_schema_create",
                                    constructor_params,
                                    c_params)

    @staticmethod
    async def deserialize(data: dict):
        try:
            # Todo: Find better way to access attr_names. Potential for issues.
            attrs = data['data']['data']['attr_names']
            schema = await Schema._deserialize(Schema,
                                               "cxs_schema_deserialize",
                                               json.dumps(data),
                                               data['source_id'],
                                               data['name'],
                                               attrs)
            return schema
        except KeyError:
            raise CxsError(ErrorCode.InvalidSchema)

    @staticmethod
    async def lookup(source_id: str, schema_no: int):
        try:
            schema = Schema(source_id, '', [])

            if not hasattr(Schema.lookup, "cb"):
                schema.logger.debug("cxs_schema_get_attributes: Creating callback")
                Schema.lookup.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

            c_source_id = c_char_p(source_id.encode('utf-8'))
            c_schema_no = c_uint32(schema_no)

            result = await do_call('cxs_schema_get_attributes',
                                   c_source_id,
                                   c_schema_no,
                                   Schema.lookup.cb)
            schema.logger.debug("created schema object")

            schema_result = json.loads(result.decode())
            schema_data = schema_result['data']['data']
            schema.attrs = schema_data['attr_names']
            schema.name = schema_data['name']
            schema.handle = schema_result['handle']
            return schema
        except KeyError:
            raise CxsError(ErrorCode.InvalidSchema)

    async def serialize(self) -> dict:
        return await self._serialize(Schema, 'cxs_schema_serialize')

    def release(self) -> None:
        self._release(Schema, 'cxs_schema_release')

