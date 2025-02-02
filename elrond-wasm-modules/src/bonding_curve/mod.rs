elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod curves;
pub mod utils;
use utils::{events, owner_endpoints, storage, user_endpoints};

#[elrond_wasm::module]
pub trait BondingCurveModule:
    storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
}
