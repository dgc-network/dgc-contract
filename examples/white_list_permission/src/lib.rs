// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate sabre_sdk;
extern crate protobuf;

mod protos;

use sabre_sdk::{WasmPtr, WasmPtrList, execute_smart_permission_entrypoint, WasmSdkError, Request};
use protos::payload::CreateProposalAction;
use protos::agent::AgentContainer;

/// Agents have a white list of agents they can send
/// proposals to.
///
fn has_permission(request: Request) -> Result<bool, WasmSdkError> {
    let proposal = protobuf::parse_from_bytes::<CreateProposalAction>(request.get_payload())?;
    let receiving_agent = proposal.get_receiving_agent();

    let agent_bytes = request.get_state(request.get_org_id())?;

    let agents = protobuf::parse_from_bytes::<AgentContainer>(&agent_bytes)?;

    let agent = agents
        .get_entries()
        .into_iter()
        .find(|agent| agent.get_public_key() == request.get_public_key());

    if let Some(a) = agent {
        Ok(a.get_whiteList()
           .to_vec()
           .iter()
           .any(|x| x == receiving_agent))
    } else {
        Ok(false)
    }
}

#[no_mangle]
pub unsafe fn entrypoint(roles: WasmPtrList, org_id: WasmPtr, public_key: WasmPtr, payload: WasmPtr) -> i32 {
    execute_smart_permission_entrypoint(roles, org_id, public_key, payload, has_permission)
}
