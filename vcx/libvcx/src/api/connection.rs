extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use utils::threadpool::spawn;
use std::ptr;
use error::ToErrorCode;
use error::connection::ConnectionError;
use connection::{get_source_id, build_connection, build_connection_with_invite, connect, to_string, get_state, release, is_valid_handle, update_state, from_string, get_invite_details, delete_connection};

/// Delete a Connection object and release its handle.
///
/// <daniel>
/// What effect does deleting the Connection object have on the state of the underlying conn that it
///   manages, as perceived by the two parties? Does it send a signal to the other party that the
///   conn is being closed? Does it remove the other party's verkey from the local wallet? Does
///   it cause the verkey for the local DID to be set to 0 so the connection is unrecoverable?
///
/// Document the preconditions:
///   - valid connection_handle
///   - valid pointer to callback
///
/// If this function fails, what cleanup is necessary?
///
/// What happens if another thread is using the connection_handle while this function is called?
///
/// Is this function idempotent (calling it multiple times on the same handle is harmless)? It looks
/// like the answer is No (assuming that is_valid_handle() will return false on something that's
/// already closed). Should it be? Document, either way.
/// </daniel>
///
/// #Params
/// command_handle: command handle to map callback to user context. <daniel>I suggest calling it a
///   "caller context" rather than a "user context".</daniel>
///
/// connection_handle: handle of the connection to delete. <daniel>Precondition: must be valid.</daniel>
///
/// cb: Callback that provides feedback of the api call. <daniel>Precondition: must be non-null.</daniel>
///
/// #Returns
/// Error code as a u32 <daniel>List common errors that could occur, if we know what they are.</daniel>
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_connection_delete_connection(command_handle: u32,
                                               connection_handle: u32,
                                               cb: Option<extern fn(
                                                   xcommand_handle: u32,
                                                   err: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    if !is_valid_handle(connection_handle) {
        return ConnectionError::InvalidHandle().to_error_code()
    }
    info!("vcx_connection_delete_connection(command_handle: {}, connection_handle: {})", command_handle, connection_handle);
    spawn(move|| {
        match delete_connection(connection_handle) {
            Ok(_) => {
                info!("vcx_connection_delete_connection_cb(command_handle: {}, rc: {})", command_handle, 0);
                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(e) => {
                info!("vcx_connection_delete_connection_cb(command_handle: {}, rc: {})", command_handle, e);
                cb(command_handle, e.to_error_code());
            },
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// -> Create a Connection object that provides a pairwise connection for an institution's user.
///
/// <daniel_notes>
/// Need to clarify ordering assumptions -- do we call this as a means of exchanging the connection
///   request/response, or as a precondition to sending such messages?
///
/// If this function fails, what cleanup is necessary?
///
/// What happens if I call this function twice on the same source_id?
/// </daniel_notes>
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user <daniel>Need link to some place
///   that describes this notion in more detail -- why we're not using DIDs, what some examples
///   of this sort of identifier might be, etc. Precondition: cannot be null. But what other
///   differences are significant? For example, is whitespace? Case? Unicode normalized vs. unnormalized
///   form?
/// </daniel>
///
/// cb: Callback that provides connection handle and error status of request<daniel>. Precondition: cannot be null.</daniel>
///
/// #Returns
/// Error code as a u32 <daniel>List common errors that could occur, if we know what they are.</daniel>
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_connection_create(command_handle: u32,
                                    source_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    info!("vcx_connection_create(command_handle: {}, source_id: {})", command_handle, source_id);
    spawn(move|| {
        match build_connection(&source_id) {
            Ok(handle) => {
                info!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error_string(0), handle, source_id);
                cb(command_handle, error::SUCCESS.code_num, handle);
            },
            Err(x) => {
                warn!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x.to_string(), 0, source_id);
                cb(command_handle, x.to_error_code(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Create a Connection object from the given invite_details that provides a pairwise connection.
/// <daniel>Need to provide some more commentary here, such as: "This function is typically called
/// by the party that *receives* a connection invitation, not the first mover that sends the invitation
/// to begin with. It is the second step in the following workflow: <hyperlink to sequence diagram>."
///
/// What can go wrong, and how are such problems handled? For example, if invite_details are
/// unsupported (bad version, malformed), do we reply with an error to the sender of the invite?
/// What cleanup do we have to do on error?
///
/// Are there any timeouts on callbacks? If not, what happens if we get into an infinite wait for
/// the connection to complete?
///
/// Is a long-running command interruptible by command handle?
/// </daniel>
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// invite_details: Provided via the other end of the connection calling "vcx_connection_connect" or "vcx_connection_invite_details"
/// <daniel>Provide an example here, inline in the comments, so people can see what it looks like.</daniel>
///
/// cb: Callback that provides connection handle and error status of request<daniel>. Precondition: cannot be null.</daniel>
///
/// #Returns
/// Error code as a u32 <daniel>List common errors that could occur, if we know what they are.</daniel>
#[no_mangle]
pub extern fn vcx_connection_create_with_invite(command_handle: u32,
                                                source_id: *const c_char,
                                                invite_details: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(invite_details, error::INVALID_OPTION.code_num);
    info!("vcx_connection_create_with_invite(command_handle: {}, source_id: {})", command_handle, source_id);
    spawn(move|| {
        match build_connection_with_invite(&source_id, &invite_details) {
            Ok(handle) => {
                info!("vcx_connection_create_with_invite_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error_string(0), handle, source_id);
                cb(command_handle, error::SUCCESS.code_num, handle);
            },
            Err(x) => {
                warn!("vcx_connection_create_with_invite_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x.to_string(), 0, source_id);
                cb(command_handle, x.to_error_code(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Establishes connection between institution and its user
///
/// <daniel>
/// This function should be renamed, or else we should not claim that it only operates between
/// institution and user; hopefully, the latter. We want connections to work the same way between
/// two people, or between institutions and other institutions, etc.
///
/// On failure, should I call vcx_connection_delete_connection()? Are (some) errors retryable?
///
/// What happens if I call this function on a connection that's already connected?
/// </daniel>
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies connection object
///
/// connection_options: Provides details indicating if the connection will be established by text or QR Code
///   <daniel>Need to clarify that null/empty string is allowed, and what it will mean if that is used
///   as the value.</daniel>
///
/// # Examples connection_options -> "{"connection_type":"SMS","phone":"123"}" OR: "{"connection_type":"QR","phone":""}"
///
/// cb: Callback that provides error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_connect(command_handle:u32,
                                     connection_handle: u32,
                                     connection_options: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, invite_details: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let options = if !connection_options.is_null() {
        check_useful_opt_c_str!(connection_options, error::INVALID_OPTION.code_num);
        connection_options.to_owned()
    }
    else {
        None
    };

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_connect(command_handle: {}, connection_handle: {}, connection_options: {:?}), source_id: {:?}",
          command_handle, connection_handle, options, source_id);

    spawn(move|| {
        match connect(connection_handle, options) {
            Ok(_) => {
                match get_invite_details(connection_handle,true) {
                    Ok(x) => {
                        info!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error_string(0), x, source_id);
                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
                    },
                    Err(e) => {
                        warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error_string(0), "null", source_id);
                        // <daniel>Explain why we would be reporting success here even though we did
                        // not find invite details. Could be fine that we are--but that raised my
                        // eyebrows when I read the code.</daniel>
                        cb(command_handle, error::SUCCESS.code_num, ptr::null_mut());
                    },
                }
            },
            Err(x) => {
                warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}, source_id: {})",
                      command_handle, connection_handle, x.to_string(), "null", source_id);
                cb(command_handle,x.to_error_code(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the Connection object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides json string of the connection's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_serialize(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, serialized_data: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_serialize(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    spawn(move|| {
        match to_string(connection_handle) {
            Ok(json) => {
                info!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(0), json, source_id);
                let msg = CStringUtils::string_to_cstring(json);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a connection object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_data: json string representing a connection object
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_deserialize(command_handle: u32,
                                      connection_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(connection_data, error::INVALID_OPTION.code_num);

    info!("vcx_connection_deserialize(command_handle: {}, connection_data: {})", command_handle, connection_data);

    spawn(move|| {
        let (rc, handle) = match from_string(&connection_data) {
            Ok(x) => {
                let source_id = get_source_id(x).unwrap_or_default();
                info!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, source_id);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {} )",
                      command_handle, error_string(x.to_error_code()), 0);
                (x.to_error_code(), 0)
            },
        };

        cb(command_handle, rc, handle);

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Checks for any state change in the connection and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_update_state(command_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_update_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    spawn(move|| {
        let rc = match update_state(connection_handle) {
            Ok(x) => {
                info!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error_string(0), connection_handle, get_state(connection_handle), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      // TODO: Refactor Error
                      command_handle, error_string(x.to_error_code()), connection_handle, get_state(connection_handle), source_id);
                x.to_error_code()
            },
        };
        let state = get_state(connection_handle);
        cb(command_handle, rc, state);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the connection object
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Connection handle that was provided during creation. Used to access connection object
///
/// cb: Callback that provides most current state of the connection and error status of request
///
/// #Returns
#[no_mangle]
pub extern fn vcx_connection_get_state(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_get_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    spawn(move|| {
        info!("vcx_connection_get_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), connection_handle, get_state(connection_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, get_state(connection_handle));

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Gets the current connection details
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// abbreviated: abbreviated connection details for QR codes or not
///
/// cb: Callback that provides the json string of details
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_invite_details(command_handle: u32,
                                            connection_handle: u32,
                                            abbreviated: bool,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, details: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    info!("vcx_connection_invite_details(command_handle: {}, connection_handle: {}, abbreviated: {}), source_id: {:?}",
          command_handle, connection_handle, abbreviated, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    spawn(move|| {
        match get_invite_details(connection_handle, abbreviated){
            Ok(str) => {
                info!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                      command_handle, connection_handle, error_string(0), str, source_id);
                let msg = CStringUtils::string_to_cstring(str);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}, source_id: {:?})",
                      command_handle, connection_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Releases the connection object by de-allocating memory
///
/// #Params
/// connection_handle: was provided during creation. Used to identify connection object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_connection_release(connection_handle: u32) -> u32 {
    let source_id = get_source_id(connection_handle).unwrap_or_default();
    match release(connection_handle) {
        Ok(_) => info!("vcx_connection_release(connection_handle: {}, rc: {}), source_id: {:?}",
                       connection_handle, error_string(0), source_id),
        Err(e) => warn!("vcx_connection_release(connection_handle: {}), rc: {}), source_id: {:?}",
                        connection_handle, error_string(e.to_error_code()), source_id),
    };

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use utils::error;
    use std::time::Duration;
    use api::VcxStateType;
    use utils::httpclient;
    use utils::constants::GET_MESSAGES_RESPONSE;
    use utils::libindy::return_types_u32;
    use utils::error::SUCCESS;

    #[test]
    fn test_vcx_connection_create() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_create(cb.command_handle,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
    }

    #[test]
    fn test_vcx_connection_create_fails() {
        init!("true");
        let rc = vcx_connection_create(0,
                                       CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_create(cb.command_handle,
                                       ptr::null(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_vcx_connection_connect() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(cb.command_handle, 0, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
        let handle = build_connection("test_vcx_connection_connect").unwrap();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(cb.command_handle,handle, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        let invite_details = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert!(invite_details.is_some());
    }

    #[test]
    fn test_vcx_connection_update_state() {
        init!("true");
        let handle = build_connection("test_vcx_connection_update_state").unwrap();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        let rc = vcx_connection_update_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_vcx_connection_update_state_fails() {
        init!("true");
        let rc = vcx_connection_update_state(0,0,None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_vcx_connection_serialize() {
        init!("true");
        let handle = build_connection("test_vcx_connection_get_data").unwrap();
        assert!(handle > 0);

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_serialize(cb.command_handle,handle, Some(cb.get_callback()));
        assert_eq!(rc, 0);

        // unwraps on the option, if none, then serializing failed and panic! ensues.
        cb.receive(Some(Duration::from_secs(10))).unwrap().unwrap();
    }

    #[test]
    fn test_vcx_connection_release() {
        init!("true");
        let handle = build_connection("test_vcx_connection_release").unwrap();
        assert!(handle > 0);

        let rc = vcx_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_vcx_connection_deserialize_succeeds() {
        init!("true");
        let string = ::utils::constants::DEFAULT_CONNECTION;
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let err = vcx_connection_deserialize(cb.command_handle,
                                                CString::new(string).unwrap().into_raw(),
                                                Some(cb.get_callback()));
        assert_eq!(err, SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle>0);
    }

    #[test]
    fn test_vcx_connection_get_state() {
        init!("true");
        let handle = build_connection("test_vcx_connection_get_state").unwrap();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        let rc = vcx_connection_update_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_get_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32)
    }

    #[test]
    fn test_vcx_connection_delete_connection() {
        init!("true");
        let test_name = "test_vcx_connection_delete_connection";
        let connection_handle = build_connection(test_name).unwrap();
        let command_handle = 0;
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(0, vcx_connection_delete_connection(command_handle, connection_handle, Some(cb.get_callback())));
    }
}
