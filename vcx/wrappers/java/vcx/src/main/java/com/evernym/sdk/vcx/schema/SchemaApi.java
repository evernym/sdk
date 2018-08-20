package com.evernym.sdk.vcx.schema;


import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class SchemaApi extends VcxJava.API {
    static String TAG = "JAVA_WRAPPER:SchemaApi ";

    private static Callback schemaCreateCB = new Callback() {
        // TODO: This callback and jna definition needs to be fixed for this API
        // it should accept connection handle as well
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, int schema_handle) {
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            Integer result = schema_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> schemmaCreate(String SourceId,String SchemaName, String SchemaDate) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(SourceId, "SourceId");
        ParamGuard.notNullOrWhiteSpace(SourceId, "SchemaName");
        ParamGuard.notNullOrWhiteSpace(SourceId, "SchemaDate");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_create(
                commandHandle,
                SourceId,
                SchemaName,
                SchemaDate,
                schemaCreateCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaSerializeHandle = new Callback() {
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

    public static CompletableFuture<String> schemaSerialize(int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_serialize(
                commandHandle,
                schemaHandle,
                schemaSerializeHandle
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaDeserializeCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, long connection_handle) {
            CompletableFuture<Long> future = (CompletableFuture<Long>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            // TODO complete with exception if we find error
//            if (err != 0) {
//                future.completeExceptionally();
//            } else {
//
//            }
            Long result = connection_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Long> schemaDeserialize(String schemaData) throws VcxException {
        ParamGuard.notNull(schemaData, "schemaData");
        CompletableFuture<Long> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_schema_deserialize(
                commandHandle,
                schemaData,
                schemaDeserializeCB
        );
        checkResult(result);
        return future;
    }

    private static Callback schemaGetAttributesCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String schemaAttributes) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaAttributes);
        }
    };

    public static CompletableFuture<String> schemaGetAttributes(int connectionHandle, String SourceId, int SequenceNo) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(SourceId, "SourceId");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_schema_get_attributes(commandHandle, SourceId,SequenceNo, schemaGetSchemaID);
        checkResult(result);
        return future;
    }

    private static Callback schemaGetSchemaID = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int command_handle, int err, String schemaId) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
            if (!checkCallback(future, err)) return;
            future.complete(schemaId);
        }
    };

    public static CompletableFuture<String> schemaGetSchemaId(int connectionHandle, int schemaHandle) throws VcxException {
        ParamGuard.notNull(schemaHandle, "SchemaHandle");
        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_schema_get_schema_id(commandHandle,schemaHandle, schemaGetSchemaID);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> schemaRelease(
            int schemaHandle
    ) throws VcxException {
        ParamGuard.notNull(schemaHandle, "schemaHandle");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();

        int result = LibVcx.api.vcx_schema_release(schemaHandle);
        checkResult(result);

        return future;
    }
}
