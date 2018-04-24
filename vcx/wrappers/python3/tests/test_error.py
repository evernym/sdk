from vcx.error import VcxError
from vcx.common import error_message


def test_c_error_msg():
    assert error_message(0) == 'Success'
    assert VcxError(0, error_message).error_msg == 'Success'
    assert VcxError(1, error_message).error_msg == 'Unknown Error'
