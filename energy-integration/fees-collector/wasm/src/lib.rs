// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           35
// Async Callback (empty):               1
// Total number of exported functions:  37

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    fees_collector
    (
        claimRewards
        addKnownContracts
        removeKnownContracts
        addKnownTokens
        removeKnownTokens
        getLockedTokenId
        getAllTokens
        getAllKnownContracts
        getLastActiveWeekForUser
        getUserEnergyForWeek
        getLastGlobalUpdateWeek
        getTotalRewardsForWeek
        getTotalEnergyForWeek
        getTotalLockedTokensForWeek
        updateEnergyForUser
        getCurrentClaimProgress
        depositSwapFees
        getAccumulatedFees
        setLockedTokensPerBlock
        getLastLockedTokensAddWeek
        getLockedTokensPerBlock
        setLockingScAddress
        setLockEpochs
        getLockingScAddress
        getLockEpochs
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        getCurrentWeek
        getFirstWeekStartEpoch
        pause
        unpause
        isPaused
        addSCAddressToWhitelist
        removeSCAddressFromWhitelist
        isSCAddressWhitelisted
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
