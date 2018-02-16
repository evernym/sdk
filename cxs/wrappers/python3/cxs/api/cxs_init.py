from ctypes import *
import logging
from cxs.common import do_call, create_cb


async def cxs_init(config_path: str) -> None:
    logger = logging.getLogger(__name__)

    if not hasattr(cxs_init, "cb"):
        logger.debug("cxs_init: Creating callback")
        cxs_init.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config_path = c_char_p(config_path.encode('utf-8'))

    result = await do_call('cxs_init',
                           c_config_path,
                           cxs_init.cb)

    logger.debug("cxs_init completed")
    return result
