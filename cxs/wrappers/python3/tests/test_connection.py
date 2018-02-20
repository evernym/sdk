import pytest
import random
from vcx.error import ErrorCode, CxsError
from vcx.state import State
from vcx.api.connection import Connection
from vcx.common import release
from ctypes import *

source_id = '123'
phone_number = '8019119191'


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_create_connection():
    connection = await Connection.create(source_id)
    assert connection.source_id == source_id
    assert connection.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_connection_connect():
    connection = await Connection.create(source_id)
    invite_details = await connection.connect(phone_number)
    assert invite_details


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_call_to_connect_with_bad_handle():
    with pytest.raises(CxsError) as e:
        invalid_connection = Connection(source_id)
        invalid_connection.handle = 0
        await invalid_connection.connect(phone_number)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_call_to_connect_state_not_initialized():
    with pytest.raises(CxsError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        data = await connection.serialize()
        data['state'] = 0
        data['handle'] = random.randint(900, 99999)
        connection2 = await Connection.deserialize(data)
        await connection2.connect(phone_number)
    assert ErrorCode.NotReady == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_serialize():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    data = await connection.serialize()
    assert connection.handle == data.get('handle')
    assert data.get('source_id') == source_id


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(CxsError) as e:
        connection = Connection(source_id)
        connection.handle = 0
        await connection.serialize()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_deserialize():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    data = await connection.serialize()
    connection2 = await Connection.deserialize(data)
    assert connection2.handle == data.get('handle')

    state = await connection2.get_state()
    assert state == State.OfferSent
    connection3 = connection

@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(CxsError) as e:
        data = {'invalid': -99}
        await Connection.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    data1 = await connection.serialize()
    connection2 = await Connection.deserialize(data1)
    data2 = await connection2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_connection_release():
    with pytest.raises(CxsError) as e:
        connection = await Connection.create(source_id)
        assert connection.handle > 0
        connection.release()
        await connection.serialize()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_update_state():
    connection = await Connection.create(source_id)
    assert await connection.update_state() == State.Initialized
    await connection.connect(phone_number)
    assert await connection.update_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(CxsError) as e:
        connection = Connection(source_id)
        connection.handle = 0
        await connection.update_state()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_get_state():
    connection = await Connection.create(source_id)
    assert await connection.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_invite_details_with_abbr():
    connection = await Connection.create(source_id)
    details = await connection.invite_details(True)
    assert details.get('s').get('dp')


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_invite_details_without_abbr():
    connection = await Connection.create(source_id)
    details = await connection.invite_details(False)
    assert details.get('senderAgencyDetail')


@pytest.mark.asyncio
@pytest.mark.usefixtures('cxs_init_test_mode')
async def test_release_called_after_gc():
    with pytest.raises(CxsError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        handle = connection.handle
        connection.__del__()
        await release('cxs_connection_release', c_uint32(handle))
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code
