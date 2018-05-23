package com.evernym.sdk.vcx.api.vcx;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.sun.jna.Callback;
import com.evernym.sdk.vcx.VcxJava;

import java9.util.concurrent.CompletableFuture;

public class Vcx extends VcxJava.API {

    private Vcx(){}

    private static Callback vcxIniWithConfigCB = new Callback() {
        public void callback(int command_handle,int err){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    private static Callback vcxInitCB = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err) {
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;
            Void result = null;
            future.complete(result);

        }
    };

    public static CompletableFuture<Integer> vcxInitWithConfig(String config_json) throws VcxException {

        ParamGuard.notNullOrWhiteSpace(config_json,"config");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init_with_config(
                commandHandle,
                config_json,
                vcxIniWithConfigCB);
        checkResult(result);

        return future;

    }
    public static CompletableFuture<Integer> vcxInit(String configPath) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configPath,"configPath");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init(
                commandHandle, configPath,
                vcxInitCB);
        checkResult(result);
        return future;
    }

    public static String vcxErrorCMessage(int errorCode) {

        return LibVcx.api.vcx_error_c_message(errorCode);


    }

    private static Callback vcxCreateConnectionWithInviteCB = new Callback() {
        public void callback(int command_handle, int err, int connectionHandle){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxCreateConnectionWithInvite(String invitationId, String inviteDetails) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(invitationId, "invitationId");
        ParamGuard.notNullOrWhiteSpace(inviteDetails, "inviteDetails");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_create_with_invite(
            commandHandle,
            invitationId,
            inviteDetails,
            vcxCreateConnectionWithInviteCB
        );
        checkResult(result);
        return future;
    }

    private static Callback vcxAcceptInvitationCB = new Callback() {
        public void callback(int command_handle, int err, String inviteDetails){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Integer result = command_handle;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> vcxAcceptInvitation(int connectionHandle, String connectionType) throws VcxException {
        ParamGuard.notNull(connectionHandle, "connectionHandle");
        ParamGuard.notNullOrWhiteSpace(connectionType, "connectionType");
        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_connection_connect(
            commandHandle,
            connectionHandle,
            connectionType,
            vcxAcceptInvitationCB
        );
        checkResult(result);
        return future;
    }

}
