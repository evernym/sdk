package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.evernym.sdk.vcx.vcx.VcxApi;

import org.awaitility.Awaitility;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import java9.util.concurrent.CompletableFuture;
import java.util.concurrent.Callable;

import org.awaitility.Awaitility.*;
import org.awaitility.Duration.*;

import java.util.concurrent.TimeUnit.*;

import org.hamcrest.Matchers.*;
import org.junit.Assert.*;

import static junit.framework.Assert.*;

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
        CompletableFuture<Integer> futureResult = ConnectionApi.vcxConnectionCreate(TestHelper.CONNECTION_ID);
        Awaitility.await().until(futureResult::isDone);

        Integer result = futureResult.getNow(-1);
        return result;
    }

    @Test
    public void createConnection() throws VcxException {

        int ConnectionHandle = _createConnection();
        assertNotSame(null,ConnectionHandle);
        assertNotSame( 0,ConnectionHandle);
    }

//    @Test
//    public void createConnectionWithoutPhone() {
//
//    }

    @Test
    public void createConnectionWithPhone() throws VcxException {
//        CompletableFuture<Integer> futureResult = ConnectionApi.vcxConnectionCreate(TestHelper.CONNECTION_ID);
//        Awaitility.await().until(futureResult::isDone);
//
//        Integer connectionHandle = futureResult.getNow(-1);
        String payload= "{ 'connection_type': 'SMS', 'phone':'1234' }";
        CompletableFuture<String> future = ConnectionApi.vcxAcceptInvitation(2,payload);
        Awaitility.await().until(future::isDone);
        assertNotSame("",future.getNow(""));


    }

//    @Test(expected = VcxException.class)
//    public void throwUninitializeError() {
//
//    }
//
//    @Test
//    public void serializeConnection() {
//
//    }


}
