// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pub type WasmPtr = i32;
pub type WasmPtrList = i32;

#[no_mangle]
extern {
    pub fn get_state(addr: WasmPtr) -> WasmPtr;
    pub fn get_ptr_len(ptr: WasmPtr) -> isize;
    pub fn get_capacity_len(ptr: WasmPtr) -> isize;
    pub fn alloc(len: usize) -> WasmPtr;
    pub fn read_byte(offset: isize) -> u8;
    pub fn write_byte(ptr: WasmPtr, offset: u32, byte: u8) -> i32;
    pub fn get_ptr_collection_len(ptr: WasmPtrList) -> isize;
    pub fn get_ptr_from_collection(ptr: WasmPtrList, index: u32) -> WasmPtr;
}
