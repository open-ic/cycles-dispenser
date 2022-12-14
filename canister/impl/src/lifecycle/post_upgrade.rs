use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use crate::{init_state, Data, State};
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::deserialize_from_stable_memory;
use tracing::info;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade() {
    ic_cdk::setup();

    let env = Box::new(CanisterEnv::new());
    let data: Data = deserialize_from_stable_memory(UPGRADE_BUFFER_SIZE).unwrap();

    let state = State::new(env, data);

    init_state(state);

    info!("Post-upgrade complete");
}
