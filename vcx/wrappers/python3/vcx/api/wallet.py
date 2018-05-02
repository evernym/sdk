from ctypes import *
from vcx.common import do_call, create_cb

import logging

class Wallet():

    @staticmethod
    async def get_token_info(handle: int) -> str:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.get_token_info, "cb"):
            logger.debug("vcx_wallet_get_token_info: Creating callback")
            Wallet.get_token_info.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_payment = c_uint32(handle)

        result = await do_call('vcx_wallet_get_token_info',
                               c_payment,
                               Wallet.get_token_info.cb)

        logger.debug("vcx_wallet_get_token_info completed")
        return result

    @staticmethod
    async def send_tokens(handle: int, tokens: float, address: str) -> str:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.send_tokens, "cb"):
            logger.debug("vcx_wallet_send_tokens: Creating callback")
            Wallet.send_tokens.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_handle = c_uint32(0)
        c_tokens = c_float(tokens)
        c_address = c_char_p(address.encode('utf-8'))

        result = await do_call('vcx_wallet_send_tokens',
                               c_handle,
                               c_tokens,
                               c_address,
                               Wallet.send_tokens.cb)

        logger.debug("vcx_wallet_send_tokens completed")
        return result
