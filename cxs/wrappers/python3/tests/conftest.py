import pytest
from cxs.api.cxs_init import cxs_init


@pytest.mark.asyncio
@pytest.fixture(scope="function")
async def init_cxs():
    await cxs_init('ENABLE_TEST_MODE')
