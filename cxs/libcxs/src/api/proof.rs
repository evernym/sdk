extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use std::thread;
use std::ptr;

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               proof_request_data: *mut c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(command_handle: u32,
                                       proof_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 { error::SUCCESS.code_num }


#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }

