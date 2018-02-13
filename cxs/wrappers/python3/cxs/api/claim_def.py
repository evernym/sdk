from ctypes import *
from cxs.common import do_call, create_cb
from cxs.error import CxsError, ErrorCode
from cxs.api.cxs_base import CxsBase

import logging
import json


class ClaimDef(CxsBase):

    def __init__(self, source_id: str, name: str, schema_no: int):
        CxsBase.__init__(self, source_id)
        self._logger = logging.getLogger(__name__)
        self._source_id = source_id
        self._schema_no = schema_no
        self._name = name
        self._handle = 0

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
        claim_def = ClaimDef(source_id, name, schema_no)

        if not hasattr(ClaimDef.create, "cb"):
            claim_def._logger.debug("cxs_claimdef_create: Creating callback")
            ClaimDef.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_schema_no = c_uint32(schema_no)
        c_revocation = c_bool(revocation)
        # default enterprise_did in config is used as issuer_did
        c_issuer_did = None

        result = await do_call('cxs_claimdef_create',
                               c_source_id,
                               c_name,
                               c_schema_no,
                               c_issuer_did,
                               c_revocation,
                               ClaimDef.create.cb)

        claim_def.handle = result
        claim_def._logger.debug("created claim_def object")
        return claim_def

    @staticmethod
    async def deserialize(data: dict):
        try:
            schema_no = data['claim_def']['ref']
            claim_def = await ClaimDef._deserialize(ClaimDef,
                                                    "cxs_claimdef_deserialize",
                                                    json.dumps(data),
                                                    data['source_id'],
                                                    data['name'],
                                                    schema_no)
            return claim_def
        except KeyError:
            raise CxsError(ErrorCode.InvalidClaimDef)

    async def serialize(self) -> dict:
        return await self._serialize(ClaimDef, 'cxs_claimdef_serialize')

    async def release(self) -> None:
        await self._release(ClaimDef, 'cxs_claimdef_release')
