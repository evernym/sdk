from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb
from cxs.state import State
from cxs.api.connection import Connection

import logging
import json


class IssuerClaim:

    def __init__(self, source_id: str, attrs: dict, schema_no: int, name: str):
        self._logger = logging.getLogger(__name__)
        self._source_id = source_id
        self._schema_no = schema_no
        self._attrs = attrs
        self._name = name
        self._handle = 0
        self._state = 0

    def __del__(self):
        # destructor
        pass

    @property
    def handle(self):
        return self._handle

    @handle.setter
    def handle(self, handle):
        self._handle = handle

    @property
    def state(self):
        return self._state

    @state.setter
    def state(self, x):
        self._state = x

    @property
    def source_id(self):
        return self._source_id

    @source_id.setter
    def source_id(self, x):
        self._source_id = x

    @staticmethod
    async def create(source_id: str, attrs: dict, schema_no: int, name: str):
        attrs = {k: [v] for k, v in attrs.items()}
        issuer_claim = IssuerClaim(source_id, attrs, schema_no, name)

        if not hasattr(IssuerClaim.create, "cb"):
            issuer_claim._logger.debug("cxs_issuer_create_claim: Creating callback")
            IssuerClaim.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_data = c_char_p(json.dumps(attrs).encode('utf-8'))
        c_schema_no = c_uint32(schema_no)
        # default enterprise_did in config is used as issuer_did
        c_issuer_did = None
        result = await do_call('cxs_issuer_create_claim',
                               c_source_id,
                               c_schema_no,
                               c_issuer_did,
                               c_data,
                               c_name,
                               IssuerClaim.create.cb)

        issuer_claim.handle = result
        issuer_claim._logger.debug("created issuer_claim object")
        issuer_claim.state = State.Initialized
        return issuer_claim

    @staticmethod
    async def deserialize(data: dict):
        issuer_claim = IssuerClaim(data.get('source_id'),
                                   data.get('claim_attributes'),
                                   data.get('schema_seq_no'),
                                   data.get('claim_request'))

        if not hasattr(IssuerClaim.deserialize, "cb"):
            issuer_claim._logger.debug("cxs_issuer_claim_deserialize: Creating callback")
            IssuerClaim.deserialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_data = c_char_p(json.dumps(data).encode('utf-8'))

        result = await do_call('cxs_issuer_claim_deserialize',
                               c_data,
                               IssuerClaim.deserialize.cb)

        issuer_claim.handle = result
        await issuer_claim.update_state()
        issuer_claim._logger.debug("created issuer_claim object")
        return issuer_claim

    async def serialize(self) -> dict:
        if not hasattr(IssuerClaim.serialize, "cb"):
            self._logger.debug("cxs_issuer_claim_serialize: Creating callback")
            IssuerClaim.serialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_handle = c_uint32(self.handle)

        data = await do_call('cxs_issuer_claim_serialize',
                             c_handle,
                             IssuerClaim.serialize.cb)
        return json.loads(data.decode())

    async def update_state(self):
        if not hasattr(IssuerClaim.update_state, "cb"):
            self._logger.debug("cxs_issuer_claim_update_state: Creating callback")
            IssuerClaim.update_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        self.state = await do_call('cxs_issuer_claim_update_state',
                                   c_handle,
                                   IssuerClaim.update_state.cb)

    async def send_offer(self, connection: Connection):
        if not hasattr(IssuerClaim.send_offer, "cb"):
            self._logger.debug("cxs_issuer_send_claim_offer: Creating callback")
            IssuerClaim.send_offer.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_claim_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('cxs_issuer_send_claim_offer',
                      c_claim_handle,
                      c_connection_handle,
                      IssuerClaim.send_offer.cb)
        self.state = State.OfferSent

    async def send_claim(self, connection: Connection):
        if not hasattr(IssuerClaim.send_claim, "cb"):
            self._logger.debug("cxs_issuer_send_claim: Creating callback")
            IssuerClaim.send_claim.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_claim_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('cxs_issuer_send_claim',
                      c_claim_handle,
                      c_connection_handle,
                      IssuerClaim.send_claim.cb)
        await self.update_state()

    async def release(self) -> None:
        if not hasattr(IssuerClaim.release, "cb"):
            self._logger.debug("cxs_claim_issuer_release: Creating callback")
            IssuerClaim.release.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        await do_call('cxs_claim_issuer_release',
                      c_handle,
                      IssuerClaim.release.cb)
