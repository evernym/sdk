package com.evernym.sdk.vcx.issuer;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class IssuerApi extends VcxJava.API {

    static String TAG = "JAVA_WRAPPER:IssuerApi ";

    private static Callback issuerCreateCredentialCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int credntialHandle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = credntialHandle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> issuerCreateCredential(String SourceId,
                                                                    String CredentialDefId,
                                                                    String IssuerId,
                                                                    String CredentialData,
                                                                    String CredentialName,
                                                                    long Price) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(SourceId, "SourceId");
        ParamGuard.notNullOrWhiteSpace(SourceId, "CredentialDefId");
        ParamGuard.notNullOrWhiteSpace(SourceId, "SchemaId");
        //TODO: Check for more mandatory params in vcx to add in PamaGuard
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_create_credential(
                commandHandle,
                SourceId,
                CredentialDefId,
                IssuerId,
                CredentialData,
                CredentialName,
                Price,
                issuerCreateCredentialCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerSendCredentialOfferCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            int result = err;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> issuerSendCredentialOffer(int CredentialOffer,
                                                                       int ConnectionHandle) throws VcxException {
        ParamGuard.notNull(CredentialOffer, "CredentialOffer");
        ParamGuard.notNull(ConnectionHandle, "ConnectionHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential_offer(
                commandHandle,
                CredentialOffer,
                ConnectionHandle,
                issuerSendCredentialOfferCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredntialUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> issuerCredntialUpdateState(int CredentialHandle) throws VcxException {
        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(commandHandle, CredentialHandle, issuerCredntialUpdateStateCB);
        checkResult(result);
        return future;
    }

    private static Callback issuerCredntialGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> issuerCredntialGetState(int CredentialHandle) throws VcxException {
        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_connection_get_state(commandHandle, CredentialHandle, issuerCredntialGetStateCB);
        checkResult(result);
        return future;
    }
    private static Callback issuerSendCredentialCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String credentialDefId) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(credentialDefId);
        }
    };

    public static CompletableFuture<String> issuerSendCredential(int CredentialHandle,
                                                                 int ConnectionHandle) throws VcxException {
        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        ParamGuard.notNull(ConnectionHandle, "ConnectionHandle");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_send_credential(
                commandHandle,
                CredentialHandle,
                ConnectionHandle,
                issuerSendCredentialCB);

        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialSerializeCB = new Callback() {
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

    public static CompletableFuture<String> issuerCredentialSerialize(int CredentialHandle) throws VcxException {
        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_serialize(
                commandHandle,
                CredentialHandle,
                issuerCredentialSerializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback issuerCredentialDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, long handle) {
            CompletableFuture<Long> future = (CompletableFuture<Long>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Long result = handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Long> issuerCredentialDeserialize(String SerializedData) throws VcxException {
        ParamGuard.notNull(SerializedData, "SerializedData");
        CompletableFuture<Long> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_credential_deserialize(
                commandHandle,
                SerializedData,
                issuerCredentialDeserializeCB
        );
        checkResult(result);
        return future;
    }



    public static CompletableFuture<Integer> issuerTerminateCredential(
            int CredentialHandle,
            int State,
            String Msg
    ) throws VcxException {
        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        ParamGuard.notNull(State, "State");
        ParamGuard.notNullOrWhiteSpace(Msg, "Msg");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_issuer_terminate_credential(
                commandHandle,
                CredentialHandle,
                State,
                Msg);
        checkResult(result);

        return future;

    }
    public static CompletableFuture<Integer> issuerCredntialRelease(
            int credentialHandle
    ) throws VcxException {
        ParamGuard.notNull(credentialHandle, "credentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_credential_release(credentialHandle);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> issuerCredentialRequest(
            int CredentialHandle,
            String CredentialRequest) throws VcxException {

        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        ParamGuard.notNull(CredentialRequest, "CredentialRequest");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_get_credential_request(
                CredentialHandle,
                CredentialRequest);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> issuerAcceptRequest(
            int CredentialHandle) throws VcxException {

        ParamGuard.notNull(CredentialHandle, "CredentialHandle");
        CompletableFuture<Integer> future = new CompletableFuture<>();

        int result = LibVcx.api.vcx_issuer_accept_credential(
                CredentialHandle);
        checkResult(result);

        return future;
    }
}
