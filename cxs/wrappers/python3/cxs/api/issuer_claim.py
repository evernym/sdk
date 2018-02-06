from typing import Optional
from ctypes import *

import logging


class IssuerClaim:

    def __init__(self, source_id: str):
        self._source_id = source_id

    def __del__(self):
        # destructor
        pass

    @staticmethod
    async def create(source_id: str):
        pass

    @staticmethod
    async def deserialize(source_id: str):
        pass

    async def connect(self):
        pass

    async def serialize(self):
        pass

    async def update_state(self):
        pass

    async def send_offer(self):
        pass

    async def send_claim(self):
        pass

