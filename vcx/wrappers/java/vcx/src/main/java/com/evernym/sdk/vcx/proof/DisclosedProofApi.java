package com.evernym.sdk.vcx.proof;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.evernym.sdk.vcx.connection.ConnectionApi;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class DisclosedProofApi extends VcxJava.API {

    private DisclosedProofApi() {
    }

    private static Callback vcxProofCreateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            int result = proof_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<CreateProofMsgIdResult> proofCreate(
            String sourceId,
            String requestedAttributes,
            String requestedPredicates,
            String name
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(requestedAttributes, "requestedAttributes");
        ParamGuard.notNull(requestedPredicates, "requestedPredicates");
        ParamGuard.notNull(name, "name");
        CompletableFuture<CreateProofMsgIdResult> future = new CompletableFuture<CreateProofMsgIdResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_create_with_request(commandHandle, sourceId, requestedAttributes, requestedPredicates, name, vcxProofCreateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofCreateWithMsgIdCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle, String proof_request) {
            CompletableFuture<CreateProofMsgIdResult> future = (CompletableFuture<CreateProofMsgIdResult>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            CreateProofMsgIdResult result = new CreateProofMsgIdResult(proof_handle, proof_request);
            future.complete(result);
        }
    };

    public static CompletableFuture<CreateProofMsgIdResult> proofCreateWithMsgId(
            String sourceId,
            int connectionHandle,
            String msgId
    ) throws VcxException {
        ParamGuard.notNull(sourceId, "sourceId");
        ParamGuard.notNull(msgId, "msgId");
        CompletableFuture<CreateProofMsgIdResult> future = new CompletableFuture<CreateProofMsgIdResult>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_create_with_msgid(commandHandle, sourceId, connectionHandle, msgId, vcxProofCreateWithMsgIdCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofUpdateStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> proofUpdateState(
            int proof_handle
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_update_state(commandHandle, proof_handle, vcxProofUpdateStateCB);
        checkResult(result);

        return future;
    }

    private static Callback proofGetRequestsCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String proof_requests) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            String result = proof_requests;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> proofGetRequests(
            int connectionHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_get_requests(commandHandle, connectionHandle, proofGetRequestsCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofGetStateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle, LibVcx.State state) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(state.ordinal());
        }
    };

    public static CompletableFuture<Integer> proofGeteState(
            int proof_handle
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_get_state(commandHandle, proof_handle, vcxProofGetStateCB);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofSerializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle, String data) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(data);
        }
    };

    public static CompletableFuture<String> proofSerialize(
            int proof_handle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_serialize(commandHandle, proof_handle, vcxProofSerializeCB);
        checkResult(result);

        return future;
    }

    private static Callback proofDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int proof_handle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = proof_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofDeserialize(
            String serializedProof
    ) throws VcxException {
        ParamGuard.notNull(serializedProof, "serializedProof");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_deserialize(commandHandle, serializedProof, proofDeserializeCB);
        checkResult(result);

        return future;
    }

    public static CompletableFuture<Integer> proofRelease(
            int proofHandle
    ) throws VcxException {
        ParamGuard.notNull(proofHandle, "proofHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_disclosed_proof_release(proofHandle);
        checkResult(result);

        return future;
    }

    private static Callback vcxProofRetrieveCredentialsCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String matching_credentials) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            String result = matching_credentials;
            future.complete(result);
        }
    };

    public static CompletableFuture<String> proofRetrieveCredentials(
            int proofHandle
    ) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_retrieve_credentials(commandHandle, proofHandle, vcxProofRetrieveCredentialsCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofGenerateCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // resolving with no error
            Integer result = 0;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofGenerate(
            int proofHandle,
            String selectedCredentials,
            String selfAttestedAttributes
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_generate_proof(commandHandle, proofHandle, selectedCredentials, selfAttestedAttributes, vcxProofGenerateCB);
        checkResult(result);

        return future;
    }


    private static Callback vcxProofSendCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // resolving with no error
            Integer result = 0;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> proofSend(
            int proofHandle,
            int connectionHandle
    ) throws VcxException {
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_disclosed_proof_send_proof(commandHandle, proofHandle, connectionHandle, vcxProofSendCB);
        checkResult(result);

        return future;
    }

}
