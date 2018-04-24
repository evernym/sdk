import pytest
from vcx.api.schema import Schema
import json


@pytest.mark.asyncio
async def test_vcx_init(vcx_init_test_mode):
    pass


@pytest.mark.asyncio
async def test_serialize_deserialize(vcx_init_test_mode):
    s = await Schema.create('sourceid', 'name', ['age', 'height'])
    print('\n')
    parsed = await s.serialize()
    print(json.dumps(parsed, indent=4))
