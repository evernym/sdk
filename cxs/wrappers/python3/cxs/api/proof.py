from typing import Optional
from ctypes import *
from cxs.common import do_call, create_cb
from cxs.api.connection import Connection

import logging
import json


class Proof:

    def __init__(self, source_id: str):
        self._logger = logging.getLogger(__name__)
        self._source_id = source_id
        self._handle = 0
        self._state = 0
        self._proof_state = 0

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
    def proof_state(self):
        return self._proof_state

    @proof_state.setter
    def proof_state(self, x):
        self._proof_state = x

    @property
    def source_id(self):
        return self._source_id

    @source_id.setter
    def source_id(self, x):
        self._source_id = x

    @staticmethod
    async def create(source_id: str,  name: str, requested_attrs: list):
        proof = Proof(source_id)

        if not hasattr(Proof.create, "cb"):
            proof._logger.debug("cxs_proof_create: Creating callback")
            Proof.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_req_predicates = c_char_p('[]'.encode('utf-8'))
        c_req_attrs = c_char_p(json.dumps(requested_attrs).encode('utf-8'))

        result = await do_call('cxs_proof_create',
                               c_source_id,
                               c_req_attrs,
                               c_req_predicates,
                               c_name,
                               Proof.create.cb)

        proof.handle = result
        proof._logger.debug("created proof object")
        return proof

    @staticmethod
    async def deserialize(data: dict):
        proof = Proof(data.get('source_id'))

        if not hasattr(Proof.deserialize, "cb"):
            proof._logger.debug("cxs_proof_deserialize: Creating callback")
            Proof.deserialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_data = c_char_p(json.dumps(data).encode('utf-8'))

        result = await do_call('cxs_proof_deserialize',
                               c_data,
                               Proof.deserialize.cb)

        proof.handle = result
        await proof.update_state()
        proof._logger.debug("created proof object")
        return proof

    async def serialize(self):
        if not hasattr(Proof.serialize, "cb"):
            self._logger.debug("cxs_proof_serialize: Creating callback")
            Proof.serialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)

        data = await do_call('cxs_proof_serialize',
                             c_proof_handle,
                             Proof.serialize.cb)
        return json.loads(data.decode())

    async def update_state(self):
        if not hasattr(Proof.update_state, "cb"):
            self._logger.debug("cxs_proof_update_state: Creating callback")
            Proof.update_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_proof_handle = c_uint32(self.handle)

        self.state = await do_call('cxs_proof_update_state',
                                   c_proof_handle,
                                   Proof.update_state.cb)

    async def request_proof(self, connection: Connection):
        if not hasattr(Proof.request_proof, "cb"):
            self._logger.debug("cxs_proof_send_request: Creating callback")
            Proof.request_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('cxs_proof_send_request',
                      c_proof_handle,
                      c_connection_handle,
                      Proof.request_proof.cb)
        await self.update_state()

    async def get_proof(self, connection: Connection) -> list:
        if not hasattr(Proof.get_proof, "cb"):
            self._logger.debug("cxs_get_proof: Creating callback")
            Proof.get_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        proof_state, proof = await do_call('cxs_get_proof',
                                           c_proof_handle,
                                           c_connection_handle,
                                           Proof.get_proof.cb)
        self.proof_state = proof_state
        return json.loads(proof.decode())

    async def release(self) -> None:
        if not hasattr(Proof.release, "cb"):
            self._logger.debug("cxs_proof_release: Creating callback")
            Proof.release.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_proof_handle = c_uint32(self.handle)

        await do_call('cxs_proof_release',
                      c_proof_handle,
                      Proof.release.cb)

