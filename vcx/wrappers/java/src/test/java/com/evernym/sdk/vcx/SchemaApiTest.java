package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.schema.SchemaApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.concurrent.ExecutionException;

public class SchemaApiTest {
    private String sourceId = "123";
    private String schemaName = "schema name";
    private String schemaVersion = "1.1.1";
    private String schemaData = "['attr1', 'attr2', 'height', 'weight']";

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            TestHelper.getResultFromFuture(VcxApi.vcxInit(TestHelper.VCX_CONFIG_TEST_MODE));
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("create a proof")
    void createSchema() throws VcxException, ExecutionException, InterruptedException {
        int schemaHandle = TestHelper.getResultFromFuture(SchemaApi.schemaCreate(sourceId, schemaName, schemaVersion,TestHelper.convertToValidJson(schemaData),0));
        assert(schemaHandle != 0 );
    }

}
