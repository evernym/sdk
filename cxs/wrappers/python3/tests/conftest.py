import pytest
from cxs.api.cxs_init import cxs_init


@pytest.mark.asyncio
@pytest.fixture(scope="module")
async def cxs_init():
    await cxs_init('ENABLE_TEST_MODE')
