import unittest
import pytest
from vcx.common import do_call, create_cb
from vcx.error import ErrorCode, CxsError
from vcx.api.vcx_init import vcx_init
from ctypes import *



@pytest.mark.asyncio
async def test_cxs_init(vcx_init_test_mode):
    pass

