package com.evernym.connectme.wrappertest;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

import com.evernym.sdk.vcx.VcxException;
import com.evernym.sdk.vcx.api.vcx.Vcx;

import java9.util.concurrent.CompletableFuture;

public class MainActivity extends AppCompatActivity {
    private static final String TAG = "MainActivity";
    // Used to load the 'native-lib' library on application startup.

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        final Button button2 = findViewById(R.id.button1);
        button2.setOnClickListener(new View.OnClickListener() {
            public void onClick(View v) {
                // Code here executes on main thread after user presses button
                try {
//                    CompletableFuture future = Vcx.vcxInit("");
                    String res = Vcx.vcxErrorMessage(0);
                    Log.d(TAG, "onClick: " + res);
                } catch (VcxException e) {
                    e.printStackTrace();
                }
            }
        });

    }


    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    public native String stringFromJNI();
}
