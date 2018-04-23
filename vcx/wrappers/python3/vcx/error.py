from enum import IntEnum

#
# class ErrorCode(IntEnum):
#     Success = 0,
#     UnknownError = 1001,
#     ConnectionError = 1002,
#     InvalidConnectionHandle = 1003,
#     InvalidConfiguration = 1004,
#     NotReady = 1005,
#     InvalidOption = 1007,
#     InvalidDid = 1008,
#     CouldNotConnect = 1010,
#     InvalidIssuerCredentialHandle = 1015,
#     InvalidJson = 1016,
#     InvalidProofHandle = 1017,
#     InvalidProof = 1023,
#     InvalidSchema = 1031,
#     UnknownLibindyError = 1035,
#     InvalidCredentialDef = 1036,
#     InvalidCredentialDefHandle = 1037,
#     InvalidSchemaHandle = 1042,
#     InvalidSchemaSequenceNumber = 1040,
#     AlreadyInitialized = 1044,
#     InvalidInviteDetails = 1045,
#     InvalidDisclosedProofHandle = 1049,
#     InvalidCredentialHandle = 1053,
#     CreateCredentialFailed = 1055,


class VcxError(Exception):
    # error_code: ErrorCode

    def __init__(self, err, message):
        self.error_code = err
        self.error_msg = message


