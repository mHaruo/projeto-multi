// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    profile_contract
    (
        init => init
        createUser => create_user
        createProject => create_project
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
