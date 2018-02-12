import pytest
from cxs.api.cxs_init import cxs_init


@pytest.mark.asyncio
@pytest.fixture(scope="function")
async def cxs_init_test_mode():
    if not hasattr(cxs_init, "open"):
        cxs_init.open = None

    if not cxs_init.open:
        await cxs_init('ENABLE_TEST_MODE')
        cxs_init.open = True
