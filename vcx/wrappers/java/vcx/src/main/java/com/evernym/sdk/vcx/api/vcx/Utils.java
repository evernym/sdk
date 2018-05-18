package com.evernym.sdk.vcx.api.vcx;

import android.util.Log;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.ParamGuard;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java.util.concurrent.ExecutionException;

import java9.util.concurrent.CompletableFuture;


/**
 * Created by abdussami on 17/05/18.
 */

public class Utils extends VcxJava.API {
    static String TAG = "JAVA_WRAPPER:UTILS ";
    public static Callback provAsyncCB = new Callback() {
        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String config) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
            if (!checkCallback(future, err)) return;

            String result = config;
            future.complete(result);
        }
    };


    public static String vcxProvisionAgent(String config) {

        ParamGuard.notNullOrWhiteSpace(config, "config");
        Log.d(TAG, "vcxProvisionAgent config received: " + config);
        String result = LibVcx.api.vcx_provision_agent(config);
        Log.d(TAG, "vcxProvisionAgent result received: " + result);

        return result;

    }

    public static CompletableFuture<String> vcxAgentProvisionAsync(String conf) throws VcxException {
        CompletableFuture<String> future = new CompletableFuture<String>();
        int commandHandle = addFuture(future);

        int result = LibVcx.api.vcx_agent_provision_async(
                commandHandle, conf,
                provAsyncCB);
        checkResult(result);
        return future;
    }

}
