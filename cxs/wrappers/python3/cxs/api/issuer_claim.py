from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb
from cxs.state import State
from cxs.api.connection import Connection
from cxs.api.cxs_base import CxsBase

import logging
import json


class IssuerClaim(CxsBase):

    def __init__(self, source_id: str, attrs: dict, schema_no: int, name: str):
        CxsBase.__init__(self, source_id)
        self._logger = logging.getLogger(__name__)
        self._schema_no = schema_no
        self._attrs = attrs
        self._name = name
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
        issuer_claim = await IssuerClaim._deserialize(IssuerClaim,
                                                      "cxs_issuer_claim_deserialize",
                                                      json.dumps(data),
                                                      data.get('source_id'),
                                                      data.get('claim_attributes'),
                                                      data.get('schema_seq_no'),
                                                      data.get('claim_request'))
        await issuer_claim.update_state()
        return issuer_claim

    async def serialize(self) -> dict:
        return await self._serialize(IssuerClaim, 'cxs_issuer_claim_serialize')

    async def update_state(self):
        if not hasattr(IssuerClaim.update_state, "cb"):
            self._logger.debug("cxs_issuer_claim_update_state: Creating callback")
            IssuerClaim.update_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        self.state = await do_call('cxs_issuer_claim_update_state',
                                   c_handle,
                                   IssuerClaim.update_state.cb)

    async def release(self) -> None:
        await self._release(IssuerClaim, 'cxs_claim_issuer_release')

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
