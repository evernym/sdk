import pytest
import json
import random
from vcx.error import ErrorCode, VcxError
from vcx.state import State, ProofState
from vcx.api.proof import Proof
from vcx.api.connection import Connection

source_id = '123'
name = 'proof name'
phone_number = '8019119191'
requested_attrs = [{'name': 'a', 'issuer_did': '8XFh8yBzrpJQmNyZzgoTqB', 'schema_seq_no': 1},
                   {'name': 'b'},
                   {'name': 'c', 'issuer_did': '77Fh8yBzrpJQmNyZzgoTqB'}]
proof_msg = '{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proof":{ "proofs":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "primary_proof":{ "eq_proof":{ "revealed_attrs":{ "name":"1139481716457488690172217916278103335" }, "a_prime":"123", "e":"456", "v":"5", "m":{ "age":"456", "height":"4532", "sex":"444" }, "m1":"5432", "m2":"211" }, "ge_proofs":[ { "u":{ "2":"6", "1":"5", "0":"7", "3":"8" }, "r":{ "1":"9", "3":"0", "DELTA":"8", "2":"6", "0":"9" }, "mj":"2", "alpha":"3", "t":{ "DELTA":"4", "1":"5", "0":"6", "2":"7", "3":"8" }, "predicate":{ "attr_name":"age", "p_type":"GE", "value":18 } } ] }, "non_revoc_proof":null } }, "aggregated_proof":{ "c_hash":"31470331269146455873134287006934967606471534525199171477580349873046877989406", "c_list":[ [ 182 ], [ 96, 49 ], [ 1 ] ] } }, "requested_proof":{ "revealed_attrs":{ "attr1_referent":[ "claim::bb929325-e8e6-4637-ba26-b19807b1f618", "Alex", "1139481716457488690172217916278103335" ] }, "unrevealed_attrs":{ }, "self_attested_attrs":{ }, "predicates":{ "predicate1_referent":"claim::bb929325-e8e6-4637-ba26-b19807b1f618" } }, "identifiers":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e", "schema_key":{ "name":"gvt", "version":"1.0", "did":"NcYxiDXkpYi6ov5FcYDi1e" }, "rev_reg_seq_no":null } }}'

@pytest.mark.asyncio
async def test_create_proof_has_libindy_error_with_no_init():
    with pytest.raises(VcxError) as e:
        await Proof.create(source_id, '', [])
        assert ErrorCode.UnknownLibindyError == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_proof():
    proof = await Proof.create(source_id, name, requested_attrs)
    assert proof.source_id == source_id
    assert proof.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    assert data.get('source_id') == source_id
    assert data.get('name') == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        proof = Proof(source_id)
        proof.handle = 0
        await proof.serialize()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    data['state'] = State.OfferSent
    proof2 = await Proof.deserialize(data)
    assert proof2.source_id == data.get('source_id')
    assert await proof2.get_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Proof.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code
    assert 'Invalid JSON string' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    proof = await Proof.create(source_id, name, requested_attrs)
    data1 = await proof.serialize()
    proof2 = await Proof.deserialize(data1)
    data2 = await proof2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_proof_release():
    with pytest.raises(VcxError) as e:
        proof = await Proof.create(source_id, name, requested_attrs)
        assert proof.handle > 0
        proof.release()
        await proof.serialize()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    proof = await Proof.create(source_id, name, requested_attrs)
    assert await proof.update_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        proof = Proof(source_id)
        proof.handle = 0
        await proof.update_state()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    proof = await Proof.create(source_id, name, requested_attrs)
    await proof.request_proof(connection)
    assert await proof.get_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    proof = await Proof.create(source_id, name, requested_attrs)
    assert await proof.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_invalid_connection():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        proof = await Proof.create(source_id, name, requested_attrs)
        connection.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_released_proof():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        proof = await Proof.create(source_id, name, requested_attrs)
        proof.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_proof_with_invalid_proof():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    data['proof'] = json.loads(proof_msg)
    data['state'] = State.Accepted
    data['proof_state'] = ProofState.Invalid
    proof2 = await Proof.deserialize(data)
    await proof2.update_state()
    proof_data = await proof2.get_proof(connection)
    assert proof2.proof_state == ProofState.Invalid
    attrs = [{"issuer_did": "NcYxiDXkpYi6ov5FcYDi1e",
              "credential_uuid": "claim::bb929325-e8e6-4637-ba26-b19807b1f618",
              "attr_info": {"name": "name", "value": "Alex", "type": "revealed"},
              "schema_key": {"name": "gvt", "version": "1.0", "did": "NcYxiDXkpYi6ov5FcYDi1e"}}]
    assert proof_data[0] == attrs[0]
