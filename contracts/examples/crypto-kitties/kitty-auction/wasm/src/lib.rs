// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    kitty_auction
    (
        setKittyOwnershipContractAddress
        createAndAuctionGenZeroKitty
        isUpForAuction
        getAuctionStatus
        getCurrentWinningBid
        createSaleAuction
        createSiringAuction
        bid
        endAuction
        callBack
    )
}
