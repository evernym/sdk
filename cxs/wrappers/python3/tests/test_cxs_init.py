import unittest
import pytest
from vcx.common import do_call, create_cb
from vcx.error import ErrorCode, CxsError
from vcx.api.cxs_init import cxs_init
from ctypes import *


class TestCxsInit(unittest.TestCase):

    def test_noop(self):
        # Makes sure we can run tests
        self.assertTrue(True)


@pytest.mark.asyncio
async def test_cxs_init(cxs_init_test_mode):
    pass


# @pytest.mark.asyncio
# async def test_cxs_init_fails_with_invalid_config_path():
#     with pytest.raises(CxsError) as e:
#         await cxs_init('bad_path')
#         assert ErrorCode.InvalidConfiguration == e.value.error_code

#
# @pytest.mark.asyncio
# async def test_do_call_init():
#     test_do_call_init.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))
#     await do_call(
#         'cxs_init',
#         c_char_p('ENABLE_TEST_MODE'.encode('utf-8')),
#         test_do_call_init.cb)
#
#
# if __name__ == '__main__':
#     unittest.main()
