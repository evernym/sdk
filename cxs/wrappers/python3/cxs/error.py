from enum import IntEnum


class ErrorCode(IntEnum):
    Success = 0,
    UnknownError = 1001,
    ConnectionError = 1002,
    InvalidConnectionHandle = 1003,
    InvalidConfiguration = 1004,
    InvalidOption = 1007,
    InvalidDid = 1008,
    UnknownLibindyError = 1035,
    AlreadyInitialized = 1044,



class CxsError(Exception):
    # error_code: ErrorCode

    def __init__(self, error_code: ErrorCode):
        self.error_code = error_code
