package com.evernym.sdk.vcx.api.vcx;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.sun.jna.Callback;
import com.evernym.sdk.vcx.VcxJava;

import java9.util.concurrent.CompletableFuture;

public class Credentail extends VcxJava.API {

    private Credential(){}

    private static Callback vcxInitCb = new Callback() {
        public void callback(int command_handle,int err){
            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(command_handle);
            if (!checkCallback(future,err)) return;
            Void result = null;
            future.complete(result);
        }
    };

//    private static Callback vcxErrorMessageCb = new Callback() {
//        public void callback(int command_handle,int err,String error_msg){
//            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(command_handle);
//            if (!checkCallback(future,err)) return;
//            String result = error_msg;
//            future.complete(result);
//        }
//    };

    public static CompletableFuture<Void> vcxInit(
            String configPath
    ) throws VcxException {
        ParamGuard.notNullOrWhiteSpace(configPath,"configPath");
        CompletableFuture<Void> future = new CompletableFuture<Void>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_init(
                commandHandle,
                configPath,
                vcxInitCb);
        checkResult(result);

        return future;

    }

    public static String vcxErrorMessage(int errorCode) throws VcxException {

        return LibVcx.api.vcx_error_c_message(
                errorCode);


    }
}
