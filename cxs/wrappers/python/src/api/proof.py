from typing import Optional
from ctypes import *

import logging


class Proof:

    def __init__(self, source_id: str):
        self._source_id = source_id

    def __del__(self):
        # destructor
        pass

    @staticmethod
    async def create(source_id: str):
        pass

    @staticmethod
    async def deserialize():
        pass

    async def serialize(self):
        pass

    async def update_state(self):
        pass

    async def request_proof(self):
        pass

    async def get_proof(self):
        pass

