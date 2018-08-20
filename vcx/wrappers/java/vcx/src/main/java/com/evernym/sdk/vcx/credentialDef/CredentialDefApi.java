package com.evernym.sdk.vcx.credentialDef;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class CredentialDefApi extends VcxJava.API {
    static String TAG = "JAVA_WRAPPER:CredentialDefApi ";

    private static Callback credentialDefCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credentialDefHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = credentialDefHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> credentialDefCreate(String SourceId,
                                                                 String CredentialName,
                                                                 String SchemaId,
                                                                 String IssuerId,
                                                                 String Tag,
                                                                 String Config,
                                                                 int PaymentHandle
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(SourceId, "SourceId");
        ParamGuard.notNullOrWhiteSpace(SourceId, "CredentialName");
        ParamGuard.notNullOrWhiteSpace(SourceId, "SchemaId");
        //TODO: Check for more mandatory params in vcx to add in PamaGuard
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credentialdef_create(
                commandHandle,
                SourceId,
                CredentialName,
                SchemaId,
                IssuerId,
                Tag,
                Config,
                PaymentHandle,
                credentialDefCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback credentialDefSerializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String serialized_data) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            String result = serialized_data;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> credentialDefSerialize(int credentialDefHandle) throws VcxException {
        ParamGuard.notNull(credentialDefHandle, "credentialDefHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credentialdef_serialize(
                commandHandle,
                credentialDefHandle,
                credentialDefSerializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback credentialDefDeserialize = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, long credntialDefHandle) {
            CompletableFuture<Long> future = (CompletableFuture<Long>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Long result = credntialDefHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Long> credentialDefSerialize(String credefntialDefData) throws VcxException {
        ParamGuard.notNull(credefntialDefData, "credefntialDefData");
        CompletableFuture<Long> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_credentialdef_deserialize(
                commandHandle,
                credefntialDefData,
                credentialDefDeserialize
        );
        checkResult(result);
        return future;
    }


    private static Callback credentialDefGetCredentialDefIdCb = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String credentialDefId) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(credentialDefId);
        }
    };

    public static CompletableFuture<String> credentialDefGetCredentialDefId(int credDefhandle) throws VcxException {
        ParamGuard.notNull(credDefhandle, "credDefhandle");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_credentialdef_get_cred_def_id(commandHandle,credDefhandle, credentialDefGetCredentialDefIdCb);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> credentialDefRelease(
            int handle
    ) throws VcxException {
        ParamGuard.notNull(handle, "handle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_credentialdef_release(handle);
        checkResult(result);

        return future;
    }
}
