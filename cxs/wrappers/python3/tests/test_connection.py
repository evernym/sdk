import pytest
from cxs.common import do_call, create_cb
from cxs.error import ErrorCode, CxsError
from cxs.api.connection import Connection
from ctypes import *


@pytest.mark.asyncio
async def test_create_connection_has_libindy_error_with_no_init():
    with pytest.raises(CxsError) as e:
        source_id = '123'
        await Connection.create(source_id)
        assert ErrorCode.UnknownLibindyError == e.value.error_code

@pytest.mark.asyncio
async def test_create_connection():
    pass