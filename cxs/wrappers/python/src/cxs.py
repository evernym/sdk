from ctypes import *

import asyncio
import sys
import itertools
import logging

_futures = {}
_futures_counter = itertools.count()


def do_call(name: str, *args):
    logger = logging.getLogger(__name__)
    logger.debug("do_call: >>> name: %s, args: %s", name, args)

    event_loop = asyncio.get_event_loop()
    future = event_loop.create_future()
    command_handle = next(_futures_counter)

    _futures[command_handle] = (event_loop, future)

    err = getattr(_cdll(), name)(command_handle,
                                 *args)

    logger.debug("do_call: Function %s returned err: %i", name, err)

    # if err != ErrorCode.Success:
    #     logger.warning("_do_call: Function %s returned error %i", name, err)
    #     future.set_exception(CxsError(ErrorCode(err)))

    logger.debug("do_call: <<< %s", future)
    return future


def _cdll() -> CDLL:
    if not hasattr(_cdll, "cdll"):
        _cdll.cdll = _load_cdll()

    return _cdll.cdll


def _load_cdll() -> CDLL:
    logger = logging.getLogger(__name__)
    logger.debug("_load_cdll: >>>")

    libcxs_prefix_mapping = {"linux": "libcxs"}
    libcxs_suffix_mapping = {"linux": ".so"}

    os_name = sys.platform
    logger.debug("_load_cdll: Detected OS name: %s", os_name)

    try:
        libcxs_prefix = libcxs_prefix_mapping[os_name]
        libcxs_suffix = libcxs_suffix_mapping[os_name]
    except KeyError:
        logger.error("_load_cdll: OS isn't supported: %s", os_name)
        raise OSError("OS isn't supported: %s", os_name)

    libcxs_name = "{0}{1}".format(libcxs_prefix, libcxs_suffix)
    logger.debug("_load_cdll: Resolved libcxs name is: %s", libcxs_name)
    tmp_path = "~/dev/cxs/cxs/libcxs/target/debug/"
    try:
        res = CDLL(tmp_path + libcxs_name)
        logger.debug("_load_cdll: <<< res: %s", res)
        return res
    except OSError as e:
        logger.error("_load_cdll: Can't load libcxs: %s", e)
        raise e