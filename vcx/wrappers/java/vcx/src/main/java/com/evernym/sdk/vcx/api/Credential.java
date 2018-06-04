package com.evernym.sdk.vcx.api;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.sun.jna.Callback;
import com.evernym.sdk.vcx.VcxJava;

import java9.util.concurrent.CompletableFuture;

public class Credential extends VcxJava.API {

    private Credential(){}

    private static Callback vcxCredentialCreateWithMsgidCB = new Callback() {
        public void callback(int command_handle,int err,int credentailHandle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = credentailHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialCreateWithMsgid(
            String sourceId,
            int connectionHandle,
            String msgId
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(sourceId,"sourceId");
        ParamGuard.notNullOrWhiteSpace(msgId,"msgId");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_create_with_msgid(
                commandHandle,
                sourceId,
                connectionHandle,
                msgId,
                vcxCredentialCreateWithMsgidCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialSendRequestCB = new Callback() {
        public void callback(int command_handle,int err,String credentail){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            String result = credentail;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialSendRequest(
            int credentialHandle,
            int connectionHandle,
            int payment_handle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_send_request(
                commandHandle,
                credentialHandle,
                connectionHandle,
                payment_handle,
                vcxCredentialSendRequestCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialSerializeCB = new Callback() {
        public void callback(int command_handle,int err,String serializedCredentail){
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            String result = serializedCredentail;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialSerialize(
            int credentailHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_serialize(commandHandle,
                credentailHandle,
                vcxCredentialSerializeCB);
        checkResult(result);

        return future;

    }

    private static Callback vcxCredentialDeserializeCB = new Callback() {
        public void callback(int command_handle,int err,int credentailHandle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = credentailHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialDeserialize(
            String serializedCredential
    ) throws VcxException {
        ParamGuard.notNull(serializedCredential,"serializedCredential");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credential_deserialize(commandHandle,
                serializedCredential,
                vcxCredentialDeserializeCB);
        checkResult(result);

        return future;

    }

}
