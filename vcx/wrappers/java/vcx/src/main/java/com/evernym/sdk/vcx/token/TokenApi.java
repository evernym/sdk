package com.evernym.sdk.vcx.token;

import com.evernym.sdk.vcx.LibVcx;
import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.VcxJava;
import com.sun.jna.Callback;

import java9.util.concurrent.CompletableFuture;

public class TokenApi extends VcxJava.API {

    private TokenApi(){}
    
      private static Callback vcxTokenCB = new Callback() {
        public void callback(int command_handle, int err, String state){
            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(command_handle);
            if(!checkCallback(future,err)) return;
            String result = state;
            future.complete(result);
        }
    };

    public static CompletableFuture<Integer> getTokenInfo(
            int paymentHandle
    ) throws VcxException {

        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_wallet_get_token_info(commandHandle, paymentHandle, vcxTokenCB);
        checkResult(result);
        return future;
    }

    public static CompletableFuture<Integer> sendTokens(
            int paymentHandle,
            int tokens,
            int recipient
    ) throws VcxException {

        CompletableFuture<Integer> future = new CompletableFuture<Integer>();
        int commandHandle = addFuture(future);
        int result = LibVcx.api.vcx_wallet_send_tokens(commandHandle, paymentHandle, tokens, recipient, vcxTokenCB);
        checkResult(result);
        return future;
    }
}