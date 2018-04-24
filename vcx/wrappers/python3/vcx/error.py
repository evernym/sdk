from enum import IntEnum

""" These will be reserved for errors thrown within the wrapper, we will need to 
coordinate what range they will fall into"""


class ErrorCode(IntEnum):
    InvalidJson = 3000


class VcxError(Exception):

    def __init__(self, err, msg_generator):
        self.error_code = err
        self.error_msg = msg_generator(err)


