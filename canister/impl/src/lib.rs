use crate::model::canisters::{CanisterMetrics, Canisters};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};
use utils::env::Environment;
use utils::memory;

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

canister_state!(State);

struct State {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl State {
    pub fn new(env: Box<dyn Environment>, data: Data) -> State {
        State { env, data }
    }

    pub fn is_caller_admin(&self) -> bool {
        self.data.admins.contains(&self.env.caller())
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            memory_used: memory::used(),
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            admins: self.data.admins.iter().copied().collect(),
            canisters: self.data.canisters.metrics(),
            max_top_up_amount: self.data.max_top_up_amount,
            min_interval: self.data.min_interval,
            min_cycles_balance: self.data.min_cycles_balance,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub admins: HashSet<Principal>,
    pub canisters: Canisters,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}

impl Data {
    pub fn new(
        admins: Vec<Principal>,
        canisters: Vec<CanisterId>,
        max_top_up_amount: Cycles,
        min_interval: Milliseconds,
        min_cycles_balance: Cycles,
        now: TimestampMillis,
    ) -> Data {
        Data {
            admins: admins.into_iter().collect(),
            canisters: Canisters::new(canisters, now),
            max_top_up_amount,
            min_interval,
            min_cycles_balance,
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub admins: Vec<Principal>,
    pub canisters: Vec<CanisterMetrics>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
}
