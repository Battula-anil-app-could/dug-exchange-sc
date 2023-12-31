// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           25
// Async Callback:                       1
// Total number of exported functions:  27

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    proxy_dex
    (
        registerProxyPair
        setTransferRoleWrappedLpToken
        registerProxyFarm
        setTransferRoleWrappedFarmToken
        getAssetTokenId
        getLockedTokenIds
        getOldLockedTokenId
        getOldFactoryAddress
        getWrappedLpTokenId
        getWrappedFarmTokenId
        addPairToIntermediate
        removeIntermediatedPair
        addFarmToIntermediate
        removeIntermediatedFarm
        getIntermediatedPairs
        getIntermediatedFarms
        addLiquidityProxy
        removeLiquidityProxy
        enterFarmProxy
        exitFarmProxy
        claimRewardsProxy
        mergeWrappedFarmTokens
        mergeWrappedLpTokens
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        callBack
    )
}
