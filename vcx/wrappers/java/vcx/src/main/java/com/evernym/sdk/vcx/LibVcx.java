package com.evernym.sdk.vcx;


import android.util.Log;

import java.io.File;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

public abstract class LibVcx {

    public static final String LIBRARY_NAME = "vcx";
    private static final String TAG ="VCX_ANDROID_WRAPPER";
	/*
     * Native library interface
	 */

    public enum vcx_state {
        none,
        initialized,
        offer_sent,
        request_received,
        accepted,
        unfulfilled,
        expired,
        revoked
    }

    public enum vcx_proof_state {
        undefined,
        validated,
        invalid
    }

    /**
     * JNA method signatures for calling SDK function.
     */
    public interface API extends Library {

        // pool.rs
        public int vcx_init_with_config(int command_handle,
                                           String config,
                                           Callback cb);
        public int vcx_init(int command_handle, String config_path, Callback cb);

        public String vcx_error_c_message(int error_code);

        public int vcx_reset();
/**
 * Schema object
 *
 * For creating, validating and committing a schema to the sovrin ledger.
 */

        /**
         * Creates a schema from a json string. Populates a handle to the new schema.
         */
        public int vcx_schema_create(int command_handle, String source_id, String schema_name, String schema_data, Callback cb);

        /**
         * Populates status with the current State of this claim.
         */
        public int vcx_schema_serialize(int command_handle, int schema_handle, Callback cb);

        /**
         * Re-creates a claim object from the specified serialization.
         */
        public int vcx_schema_deserialize(int command_handle, String serialized_schema, Callback cb);

        /**
         * Populates data with the contents of the schema handle.
         */
        public int vcx_schema_get_attributes(int command_handle, String source_id, int sequence_no, Callback cb);

        /**
         * Populates sequence_no with the actual sequence number of the schema on the sovrin ledger.
         */
        public int vcx_schema_get_sequence_no(int command_handle, int schema_handle, Callback cb);

        /**
         * Release memory associated with schema object.
         */
        public int vcx_schema_release(int handle);


/**
 * claimdef object
 *
 * For creating, validating and committing a claim definition to the sovrin ledger.
 */

        /**
         * Creates a claim definition from the given schema.  Populates a handle to the new claimdef.
         */
        public int vcx_claimdef_create(int command_handle, String source_id, String claimdef_name, int schema_seq_no, int revocation, Callback cb);

        /**
         * Populates status with the current State of this claim.
         */
        public int vcx_claimdef_serialize(int command_handle, int claimdef_handle, Callback cb);

        /**
         * Re-creates a claim object from the specified serialization.
         */
        public int vcx_claimdef_deserialize(int command_handle, String serialized_claimdef, Callback cb);

        /**
         * Asynchronously commits the claimdef to the ledger.
         */
        public int vcx_claimdef_commit(int claimdef_handle);

        /**
         * Populates sequence_no with the actual sequence number of the claimdef on the sovrin ledger.
         */
        public int vcx_claimdef_get_sequence_no(int claimdef_handle, int sequence_no);

        /**
         * Populates data with the contents of the claimdef handle.
         */
        public int vcx_claimdef_get(int claimdef_handle, String data);


/**
 * connection object
 *
 * For creating a connection with an identity owner for interactions such as exchanging
 * claims and proofs.
 */

        /**
         * Creates a connection object to a specific identity owner. Populates a handle to the new connection.
         */
        public int vcx_connection_create(int command_handle, String source_id, Callback cb);

        /**
         * Asynchronously request a connection be made.
         */
        public int vcx_connection_connect(int command_handle, int connection_handle, String connection_type, Callback cb);

        /**
         * Returns the contents of the connection handle or null if the connection does not exist.
         */
        public int vcx_connection_serialize(int command_handle, int connection_handle, Callback cb);

        /**
         * Re-creates a connection object from the specified serialization.
         */
        public int vcx_connection_deserialize(int command_handle, String serialized_claim, Callback cb);

        /**
         * Request a State update from the agent for the given connection.
         */
        public int vcx_connection_update_state(int command_handle, int connection_handle, Callback cb);

        /**
         * Retrieves the State of the connection
         */
        public int vcx_connection_get_state(int command_handle, int connection_handle, Callback cb);

        /**
         * Releases the connection from memory.
         */
        public int vcx_connection_release(int connection_handle);

        /**
         * Get the invite details for the connection.
         */
        public int vcx_connection_invite_details(int command_handle, int connection_handle, int abbreviated, Callback cb);

        /**
         * Creates a connection from the invite details.
         */
        public int vcx_connection_create_with_invite(int command_handle, String source_id, String invite_details, Callback cb);


/**
 * claim issuer object
 *
 * Used for offering and managing a claim with an identity owner.
 */

        /**
         * Creates a claim object from the specified claimdef handle. Populates a handle the new claim.
         */
        public int vcx_issuer_create_claim(int command_handle, String source_id, int schema_seq_no, String issuer_did, String claim_data, String claim_name, Callback cb);

        /**
         * Asynchronously sends the claim offer to the connection.
         */
        public int vcx_issuer_send_claim_offer(int command_handle, int claim_handle, int connection_handle, Callback cb);

        /**
         * Updates the State of the claim from the agency.
         */
        public int vcx_issuer_claim_update_state(int command_handle, int claim_handle, Callback cb);

        /**
         * Retrieves the State of the issuer_claim.
         */
        public int vcx_issuer_claim_get_state(int command_handle, int claim_handle, Callback cb);

        /**
         * Asynchronously send the claim to the connection. Populates a handle to the new transaction.
         */
        public int vcx_issuer_send_claim(int command_handle, int claim_handle, int connection_handle, Callback cb);

        /**
         * Populates status with the current State of this claim.
         */
        public int vcx_issuer_claim_serialize(int command_handle, int claim_handle, Callback cb);

        /**
         * Re-creates a claim object from the specified serialization.
         */
        public int vcx_issuer_claim_deserialize(int command_handle , String serialized_claim, Callback cb);

        /**
         * Terminates a claim for the specified reason.
         */
        public int vcx_issuer_terminate_claim(int command_handle, int claim_handle, vcx_state state_type, String msg);

        /**
         * Releases the claim from memory.
         */
        public int vcx_issuer_claim_release(int claim_handle);

        /**
         * Populates claim_request with the latest claim request received. (not in MVP)
         */
        public int vcx_issuer_get_claim_request(int claim_handle, String claim_request);

        /**
         * Sets the claim request in an accepted State. (not in MVP)
         */
        public int vcx_issuer_accept_claim(int claim_handle);

/**
 * proof object
 *
 * Used for requesting and managing a proof request with an identity owner.
 */

        /**
         * Creates a proof object.  Populates a handle to the new proof.
         */
        public int vcx_proof_create(int command_handle, String source_id, String requested_attrs, String requested_predicates, String name, Callback cb);

        /**
         * Asynchronously send a proof request to the connection.
         */
        public int vcx_proof_send_request(int command_handle, int proof_handle, int connection_handle, Callback cb);

        /**
         * Populate response_data with the latest proof offer received.
         */
        public int vcx_get_proof(int command_handle, int proof_handle, int connection_handle, Callback cb);

        /**
         * Set proof offer as accepted.
         */
        public int vcx_proof_accepted(int proof_handle);

        /**
         * Populates status with the current State of this proof request.
         */
        public int vcx_proof_update_state(int command_handle, int proof_handle, Callback cb);

        /**
         * Retrieves the State of the proof.
         */
        public int vcx_proof_get_state(int command_handle, int proof_handle, Callback cb);

        /**
         * Populates status with the current State of this proof.
         */
        public int vcx_proof_serialize(int command_handle, int proof_handle, Callback cb);

        /**
         * Re-creates a proof object from the specified serialization.
         */
        public int vcx_proof_deserialize(int command_handle, String serialized_proof, Callback cb);

        /**
         * Releases the proof from memory.
         */
        public int vcx_proof_release(int proof_handle);

/**
 * disclosed_proof object
 *
 * Used for sending a disclosed_proof to an identity owner.
 */

        /**
         * Creates a disclosed_proof object.  Populates a handle to the new disclosed_proof.
         */
        public int vcx_disclosed_proof_create(int command_handle, String source_id, String requested_attrs, String requested_predicates, String name, Callback cb);

        /**
         * Asynchronously send a proof to the connection.
         */
        public int vcx_disclosed_proof_send_proof(int command_handle, int proof_handle, int connection_handle, Callback cb);

        /**
         * Populates status with the current State of this disclosed_proof request.
         */
        public int vcx_disclosed_proof_update_state(int command_handle, int proof_handle, Callback cb);

        /**
         * Check for any proof requests from the connection.
         */
        public int vcx_disclosed_proof_get_requests(int command_handle, int connection_handle, Callback cb);

        /**
         * Retrieves the State of the disclosed_proof.
         */
        public int vcx_disclosed_proof_get_state(int command_handle, int proof_handle, Callback cb);

        /**
         * Populates status with the current State of this disclosed_proof.
         */
        public int vcx_disclosed_proof_serialize(int command_handle, int proof_handle, Callback cb);

        /**
         * Re-creates a disclosed_proof object from the specified serialization.
         */
        public int vcx_disclosed_proof_deserialize(int command_handle, String serialized_proof, Callback cb);

        /**
         * Releases the disclosed_proof from memory.
         */
        public int vcx_disclosed_proof_release(int proof_handle);

/**
 * claim object
 *
 * Used for accepting and requesting a claim with an identity owner.
 */

        /**
         * Creates a claim object from the specified claimdef handle. Populates a handle the new claim.
         */
        public int vcx_claim_create_with_offer(int command_handle, String source_id, String claim_offer, Callback cb);

        /**
         * Asynchronously sends the claim request to the connection.
         */
        public int vcx_claim_send_request(int command_handle, int claim_handle, int connection_handle, Callback cb);

        /**
         * Check for any claim offers from the connection.
         */
        public int vcx_claim_get_offers(int command_handle, int connection_handle, Callback cb);

        /**
         * Updates the State of the claim from the agency.
         */
        public int vcx_claim_update_state(int command_handle, int claim_handle, Callback cb);

        /**
         * Retrieves the State of the claim - including storing the claim if it has been sent.
         */
        public int vcx_claim_get_state(int command_handle, int claim_handle, Callback cb);

        /**
         * Populates status with the current State of this claim.
         */
        public int vcx_claim_serialize(int command_handle, int claim_handle, Callback cb);

        /**
         * Re-creates a claim from the specified serialization.
         */
        public int vcx_claim_deserialize(int command_handle, String serialized_claim, Callback cb);

        /**
         * Releases the claim from memory.
         */
        public int vcx_claim_release(int claim_handle);

/**
 * Utils object
 *
 */
        public String vcx_provision_agent(String json);

        public int vcx_agent_provision_async(int command_handle, String json,Callback cb);

        public int vcx_agent_update_info(int command_handle,String json,Callback cb);

        public int vcx_ledger_get_fees(int command_handle, Callback cb);

        public void vcx_set_next_agency_response(int message_index);

        /**
         * credential object
         *
         * Used for accepting and requesting a credential with an identity owner.
         */

        /** Creates a credential object from the specified credentialdef handle. Populates a handle the new credential. */
        public int vcx_credential_create_with_offer(int command_handle, String source_id, String credential_offer,Callback cb);

        /** Creates a credential object from the connection and msg id. Populates a handle the new credential. */
        public int vcx_credential_create_with_msgid(int command_handle, String source_id, int connection, String msg_id,Callback cb);

        /** Asynchronously sends the credential request to the connection. */
        public int vcx_credential_send_request(int command_handle, int credential_handle, int connection_handle,int payment_handle, Callback cb);

        /** Check for any credential offers from the connection. */
        public int vcx_credential_get_offers(int command_handle, int connection_handle,Callback cb);

        /** Updates the State of the credential from the agency. */
        public int vcx_credential_update_state(int command_handle, int credential_handle,Callback cb);

        /** Retrieves the State of the credential - including storing the credential if it has been sent. */
        public int vcx_credential_get_state(int command_handle, int credential_handle, Callback cb);

        /** Populates status with the current State of this credential. */
        public int vcx_credential_serialize(int command_handle, int credential_handle, Callback cb);

        /** Re-creates a credential from the specified serialization. */
        public int vcx_credential_deserialize(int command_handle, String serialized_credential, Callback cb);

        /** Releases the credential from memory. */
        public int vcx_credential_release(int credential_handle);

    }

	/*
	 * Initialization
	 */

    public static API api = null;

    static {

        try {

            init();
        } catch (UnsatisfiedLinkError ex) {

            Log.e(TAG, "static initializer: ", ex );
            // Library could not be found in standard OS locations.
            // Call init(File file) explicitly with absolute library path.
        }
    }

    /**
     * Initializes the API with the path to the C-Callable library.
     *
     * @param searchPath The path to the directory containing the C-Callable library file.
     */
    public static void init(String searchPath, String libraryName) {

        NativeLibrary.addSearchPath(libraryName, searchPath);
        api = Native.loadLibrary(libraryName, API.class);
    }

    /**
     * Initializes the API with the path to the C-Callable library.
     * Warning: This is not platform-independent.
     *
     * @param file The absolute path to the C-Callable library file.
     */
    public static void init(File file) {

        api = Native.loadLibrary(file.getAbsolutePath(), API.class);
    }

    /**
     * Initializes the API with the default library.
     */
    public static void init() {
        api = Native.loadLibrary(LIBRARY_NAME, API.class);
    }

    public static void initByLibraryName(String libraryName) {
//		System.loadLibrary("vcx");
        System.loadLibrary(libraryName);
        api = Native.loadLibrary(libraryName, API.class);
    }

    /**
     * Indicates whether or not the API has been initialized.
     *
     * @return true if the API is initialize, otherwise false.
     */
    public static boolean isInitialized() {

        return api != null;
    }
}