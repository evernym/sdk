package com.example.lodder.vcxtest;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;

import com.sun.jna.Platform;
import com.sun.jna.Native;


public class MainActivity extends AppCompatActivity {

    // Used to load the 'native-lib' library on application startup.
    private final VCXJniHandler handler = new VCXJniHandler();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        try {
//            System.setProperty("RUST_BACKTRACE", "full");
            System.loadLibrary("vcx");
            Native.register(MainActivity.class, "vcx");
        } catch (UnsatisfiedLinkError e) {
            Log.e("FAIL", e.getMessage());
        }

        // Example of a call to a native method
        TextView tv = (TextView) findViewById(R.id.sample_text);
        tv.setText(vcx_version());
        int result = vcx_agent_provision_async(10, "{\"agency_url\": \"http://10.4.32.50:9001\", \"agency_did\": \"sFJZSHGFnsTBwFUeiV83q\",\"wallet_name\":\"wallet1\",\"wallet_key\":\"wallet-key\",\"agent_seed\":null,\"enterprise_seed\":null, \"agency_verkey\": \"UPPrbEH7WRSCdaDdgoUNX8jByvi59cHwHcEr1QESrgT\"}", handler);
        Log.d("HELP", result + "");
    }

    public native String vcx_version();

    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    public native int vcx_agent_provision_async(int handle, String json, VCXJniHandler callbackHandler);
}
