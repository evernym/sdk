from ctypes import *
from cxs.common import do_call, create_cb
from cxs.error import CxsError, ErrorCode

import logging
import json


class ClaimDef:

    def __init__(self, source_id: str, name: str, schema_no: int):
        self._logger = logging.getLogger(__name__)
        self._source_id = source_id
        self._schema_no = schema_no
        self._name = name
        self._handle = 0

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
            claim_def = ClaimDef(data['source_id'], data['name'], schema_no)

            if not hasattr(ClaimDef.deserialize, "cb"):
                claim_def._logger.debug("cxs_claimdef_deserialize: Creating callback")
                ClaimDef.deserialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

            c_data = c_char_p(json.dumps(data).encode('utf-8'))

            result = await do_call('cxs_claimdef_deserialize',
                                   c_data,
                                   ClaimDef.deserialize.cb)

            claim_def.handle = result
            claim_def._logger.debug("created claim_def object")
            return claim_def
        except KeyError:
            raise CxsError(ErrorCode.InvalidClaimDef)

    async def serialize(self) -> dict:
        if not hasattr(ClaimDef.serialize, "cb"):
            self._logger.debug("cxs_claimdef_serialize: Creating callback")
            ClaimDef.serialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_handle = c_uint32(self.handle)

        data = await do_call('cxs_claimdef_serialize',
                             c_handle,
                             ClaimDef.serialize.cb)
        return json.loads(data.decode())

    async def release(self) -> None:
        if not hasattr(ClaimDef.release, "cb"):
            self._logger.debug("cxs_claimdef_release: Creating callback")
            ClaimDef.release.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        await do_call('cxs_claimdef_release',
                      c_handle,
                      ClaimDef.release.cb)
