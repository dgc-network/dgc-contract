// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate sabre_sdk;
extern crate protobuf;

mod protos;

use sabre_sdk::{WasmPtr, WasmPtrList, execute_smart_permission_entrypoint, WasmSdkError, Request};

/// Uses agent role to decide in a proposal can be created.
fn has_permission(request: Request) -> Result<bool, WasmSdkError> {
    Ok(request
       .get_roles()
       .iter()
       .any(|x| x == "proposal_creation"))
}

#[no_mangle]
pub unsafe fn entrypoint(roles: WasmPtrList, org_id: WasmPtr, payload: WasmPtr, public_key: WasmPtr) -> i32 {
    execute_smart_permission_entrypoint(roles, org_id, public_key, payload, has_permission)
}
