import pytest
from vcx.error import ErrorCode, VcxError
from vcx.common import error_message
from vcx.api.wallet import *
from ctypes import *

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_token_info():
    info = await Wallet.get_token_info(0)
    assert info

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_tokens():
    receipt = await Wallet.send_tokens(0,50.0,"address")
    assert receipt
