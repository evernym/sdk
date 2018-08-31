package com.evernym.sdk.vcx;


import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.connection.InvalidConnectionHandleException;
import com.evernym.sdk.vcx.vcx.VcxApi;
import java9.util.concurrent.CompletableFuture;
import org.awaitility.Awaitility;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static junit.framework.Assert.assertNotSame;

public class ConnectionApiTest {

    @Before
    public void setup() throws VcxException, ExecutionException, InterruptedException {

        if (!TestHelper.vcxInitialized) {
            CompletableFuture<Integer> result = VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE);
            result.get();
            TestHelper.vcxInitialized = true;
        }

    }

    private int _createConnection() throws VcxException {
        CompletableFuture<Integer> futureResult = ConnectionApi.vcxConnectionCreate(TestHelper.getConnectionId());
        Awaitility.await().until(futureResult::isDone);

        Integer result = futureResult.getNow(-1);
        if(result == -1){
            throw new VcxException("Unable to create connection handle",0);
        }else{
//            System.out.println("Connection created with connection handle => "  + result);
            return result;
        }

    }

    @Test
    public void createConnection() throws VcxException {

        long connectionHandle = _createConnection();
        assertNotSame(null,connectionHandle);
        assertNotSame( 0,connectionHandle);
    }

    @Test
    public void connectConnectionWithoutPhone() throws VcxException {
        String payload= "{ 'connection_type': 'SMS' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));
    }

    @Test
    public void connectConnectionWithPhone() throws VcxException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));


    }

    @Test(expected = InvalidConnectionHandleException.class)
    public void throwInvalidConnectionHandleException() throws VcxException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(8765,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));
    }

    @Test
    public void serializeConnection() throws VcxException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
        Awaitility.await().until(future::isDone);
        String serializedJson = future.getNow("");
        System.out.println(serializedJson);
        assertNotSame("",serializedJson);
        assert(serializedJson.contains("version"));
        assert(serializedJson.contains("data"));
    }

    @Test(expected = InvalidConnectionHandleException.class)
    public void serializeConnectionWithBadHandle() throws VcxException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.connectionSerialize(0);
        Awaitility.await().until(future::isDone);
    }

    @Test
    public void deleteConnection() throws VcxException, ExecutionException, InterruptedException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<Integer> futureDelete= ConnectionApi.deleteConnection(connectionHandle);
        Awaitility.await().until(futureDelete::isDone);
        assert(futureDelete.get() == 0);
    }

    @Test(expected = InvalidConnectionHandleException.class)
    public void serlializeDeletedConnection() throws VcxException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<Integer> futureDelete= ConnectionApi.deleteConnection(connectionHandle);
        Awaitility.await().until(futureDelete::isDone);
        CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
        Awaitility.await().until(future::isDone);
    }

    @Test(expected = InvalidConnectionHandleException.class)
    public void serlializeReleasedConnection() throws VcxException {
        Integer connectionHandle = _createConnection();
        int releaseResult= ConnectionApi.connectionRelease(connectionHandle);
        assert(releaseResult == 0 );
        CompletableFuture<String> future = ConnectionApi.connectionSerialize(connectionHandle);
        Awaitility.await().until(future::isDone);
    }

    @Test
    public void releaseConnection() throws VcxException, ExecutionException, InterruptedException {
        Integer connectionHandle = _createConnection();
        int result= ConnectionApi.connectionRelease(connectionHandle);
        assert(result == 0 );
    }

    @Test
    public void initialiseConnection() throws VcxException, ExecutionException, InterruptedException {
        Integer connectionHandle = _createConnection();
        CompletableFuture<Integer> futureUpdateState= ConnectionApi.vcxConnectionUpdateState(connectionHandle);
        Awaitility.await().until(futureUpdateState::isDone);
        int updateStateResult = futureUpdateState.get();
        assert(updateStateResult== 1 );
        CompletableFuture<Integer> futureGetState= ConnectionApi.connectionGetState(connectionHandle);
        Awaitility.await().until(futureGetState::isDone);
        assert(futureGetState.get()== updateStateResult);

    }
    @Test
    public void sendOfferConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        Integer connectionHandle = _createConnection();
        CompletableFuture<String> future = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(future::isDone);
        CompletableFuture<Integer> futureGetState= ConnectionApi.connectionGetState(connectionHandle);
        Awaitility.await().until(futureGetState::isDone);
        int connectionState = futureGetState.get();
        assert(connectionState == 2);
    }

    @Test
    public void inviteDetailsAbbreviatedConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        int connectionHandle = _createConnection();
        CompletableFuture<String> acceptInvitation = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(acceptInvitation::isDone);
        CompletableFuture<String> detials = ConnectionApi.connectionInviteDetails(connectionHandle,1);
        Awaitility.await().until(detials::isDone);
        assert(detials.get().contains("dp"));

    }

    @Test
    public void inviteDetailsUnAbbreviatedConnection() throws VcxException, ExecutionException, InterruptedException {
        String payload= "{ 'connection_type': 'SMS', 'phone':'7202200000' }";
        int connectionHandle = _createConnection();
        CompletableFuture<String> acceptInvitation = ConnectionApi.vcxConnectionConnect(connectionHandle,TestHelper.convertToValidJson(payload));
        Awaitility.await().until(acceptInvitation::isDone);
        CompletableFuture<String> detials = ConnectionApi.connectionInviteDetails(connectionHandle,0);
        Awaitility.await().until(detials::isDone);
        assert(detials.get().contains("senderAgencyDetail"));

    }


}
