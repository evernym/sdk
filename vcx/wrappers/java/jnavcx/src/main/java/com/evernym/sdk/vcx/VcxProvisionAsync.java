package com.evernym.sdk.vcx;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Platform;

import java.util.concurrent.CompletableFuture;

/** Simple example of JNA interface mapping and usage. */
public class VcxProvisionAsync {

	public static final String LIBRARY_NAME = "vcx";

    // This is the standard, stable way of mapping, which supports extensive
    // customization and mapping of Java to native types.

    public interface VcxLibrary extends Library {
    	VcxLibrary INSTANCE = (VcxLibrary)
            Native.loadLibrary(LIBRARY_NAME, VcxLibrary.class);

        public int vcx_init(int command_handle, String config_path, Callback cb);
        
        //void (*cb)(vcx_command_handle_t command_handle, vcx_error_t err, const char *config)
        public int vcx_agent_provision_async(int command_handle, String cjson, Callback cb);

    }

    public static Callback provisionCB = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String config) {
        	System.out.println("The config callback parameter is: " + config);
//            CompletableFuture<Integer> future = (CompletableFuture<Integer>) removeFuture(xcommand_handle);
//            if (!checkCallback(future, err)) return;
//
//            int result = connection_handle;
//            future.complete(result);

        }
    };
    
    public static void main(String[] args) {
    	String config="{\"agency_url\": \"http://localhost:8081\", \"agency_did\": \"sFJZSHGFnsTBwFUeiV83q\",\"wallet_name\":\"wallet1\",\"wallet_key\":\"wallet-key\",\"agent_seed\":null,\"enterprise_seed\":null, \"agency_verkey\": \"UPPrbEH7WRSCdaDdgoUNX8jByvi59cHwHcEr1QESrgT\"}";
        
    	VcxLibrary.INSTANCE.vcx_agent_provision_async(0, config, provisionCB);
//        for (int i=0;i < args.length;i++) {
//        	VcxLibrary.INSTANCE.printf("Argument %d: %s\n", i, args[i]);
//        }
    }
}