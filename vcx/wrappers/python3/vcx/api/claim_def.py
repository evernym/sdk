from ctypes import *
from vcx.error import CxsError, ErrorCode
from vcx.api.vcx_base import VcxBase

import json

HANDLES = {}


class ClaimDef(VcxBase):

    def __init__(self, source_id: str, name: str, schema_no: int):
        VcxBase.__init__(self, source_id)
        self._source_id = source_id
        self._schema_no = schema_no
        self._name = name

    def __del__(self):
        print('calling __del__')
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(ClaimDef, self.handle))

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, x):
        self._name = x

    @property
    def schema_no(self):
        return self._schema_no

    @schema_no.setter
    def schema_no(self, x):
        self._schema_no = x

    @staticmethod
    async def create(source_id: str, name: str, schema_no: int, revocation: bool):
        constructor_params = (source_id, name, schema_no)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_schema_no = c_uint32(schema_no)
        # default enterprise_did in config is used as issuer_did
        c_issuer_did = None
        c_revocation = c_bool(revocation)
        c_params = (c_source_id, c_name, c_schema_no, c_issuer_did, c_revocation)

        return await ClaimDef._create("cxs_claimdef_create",
                                      constructor_params,
                                      c_params)

    @staticmethod
    async def deserialize(data: dict):
        try:
            schema_no = data['claim_def']['ref']
            claim_def = await ClaimDef._deserialize("cxs_claimdef_deserialize",
                                                    json.dumps(data),
                                                    data['source_id'],
                                                    data['name'],
                                                    schema_no)
            return claim_def
        except KeyError:
            raise CxsError(ErrorCode.InvalidClaimDef)

    async def serialize(self) -> dict:
        return await self._serialize(ClaimDef, 'cxs_claimdef_serialize')

    def release(self) -> None:

        self._release(ClaimDef, 'cxs_claimdef_release')
