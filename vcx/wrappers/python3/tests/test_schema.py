import pytest
from vcx.error import ErrorCode, CxsError
from vcx.api.schema import Schema

source_id = '123'
name = 'schema name'
attr_names = ['attr1', 'attr2', 'height', 'weight']


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_schema():
    schema = await Schema.create(source_id, name, attr_names)
    assert schema.source_id == source_id
    assert schema.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_sets_schema_attrs():
    schema = await Schema.create(source_id, name, attr_names)
    assert schema.attrs == attr_names


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    schema = await Schema.create(source_id, name, attr_names)
    data = await schema.serialize()
    assert data.get('handle') == schema.handle
    assert data.get('source_id') == source_id
    assert data.get('name') == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(CxsError) as e:
        schema = Schema(source_id, name, attr_names)
        schema.handle = 0
        await schema.serialize()
    assert ErrorCode.InvalidSchemaHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    schema = await Schema.create(source_id, name, attr_names)
    data = await schema.serialize()
    schema2 = await Schema.deserialize(data)
    assert schema2.handle == data.get('handle')


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(CxsError) as e:
        data = {'invalid': -99}
        await Schema.deserialize(data)
    assert ErrorCode.InvalidSchema == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    schema = await Schema.create(source_id, name, attr_names)
    data1 = await schema.serialize()
    schema2 = await Schema.deserialize(data1)
    data2 = await schema2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_release():
    with pytest.raises(CxsError) as e:
        schema = await Schema.create(source_id, name, attr_names)
        assert schema.handle > 0
        schema.release()
        await schema.serialize()
    assert ErrorCode.InvalidSchemaHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_lookup():
    schema_no = 999
    schema = await Schema.lookup(source_id, schema_no)
    assert schema.attrs == ['test', 'get', 'schema', 'attrs']
    assert schema.name == 'get schema attrs'
    assert schema.handle > 0
