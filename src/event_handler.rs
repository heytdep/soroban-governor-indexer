use rs_zephyr_sdk::{
    stellar_xdr::next::{ContractEventV0, Hash, ScVal},
    utils, EnvClient,
};

use crate::db;

pub struct EventTypes {
    pub vote_cast: ScVal,
    pub proposal_created: ScVal,
    pub proposal_updated: ScVal,
}

impl EventTypes {
    pub fn new() -> Self {
        Self {
            vote_cast: utils::to_scval_symbol("vote_cast").unwrap(),
            proposal_created: utils::to_scval_symbol("proposal_created").unwrap(),
            proposal_updated: utils::to_scval_symbol("proposal_updated").unwrap(),
        }
    }
}

/// Handle a Vote Cast event
///
/// Returns None if the event is not a vote_cast event, or the data was malormed
///
/// Event:
/// - topics - `["vote_cast", proposal_id: u32, voter: Address]`
/// - data - `[support: u32, amount: i128]`
pub fn handle_vote_cast(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: ScVal,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let voter = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };

    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let support = match data.get(0).cloned() {
                Some(data) => data,
                None => return,
            };
            let amount = match data.get(1).cloned() {
                Some(data) => data,
                None => return,
            };

            let votes = db::Votes {
                contract: contract_id,
                prop_num: proposal_number,
                user: voter,
                support,
                amount,
                ledger: ledger_sequence,
            };
            db::write_votes(env, votes);
        }
    }
}

/// Handle a Proposal Created event
///
/// Returns None if the event is not a proposal_created event, or the data was malormed
///
/// - topics - `["proposal_created", proposal_id: u32, proposer: Address]`
/// - data - `[title: String, desc: String, action: ProposalAction]`
pub fn handle_proposal_created(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: ScVal,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let proposer = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };
    if let ScVal::Vec(data_opt) = &event.data {
        if let Some(data) = data_opt {
            let title = match data.get(0).cloned() {
                Some(data) => data,
                None => return,
            };
            let desc = match data.get(1).cloned() {
                Some(data) => data,
                None => return,
            };
            let action = match data.get(2).cloned() {
                Some(data) => data,
                None => return,
            };

            let proposal = db::Proposal {
                contract: contract_id,
                prop_num: proposal_number,
                title,
                desc,
                action,
                creator: proposer,
                status: ScVal::U32(0),
                ledger: ledger_sequence,
            };
            db::write_proposal(env, proposal);
        }
    }
}

/// Handle a Proposal Updated event
///
/// Returns None if the event is not a proposal_created event, or the data was malormed
///
/// - topics - `["proposal_updated", proposal_id: u32, status: u32]`
/// - data - `[]`
pub fn handle_proposal_updated(
    env: &EnvClient,
    contract_id: Hash,
    event: &ContractEventV0,
    ledger_sequence: ScVal,
) {
    let proposal_number = match event.topics.get(1).cloned() {
        Some(topic) => topic,
        None => return,
    };
    let status = match event.topics.get(2).cloned() {
        Some(topic) => topic,
        None => return,
    };

    db::update_proposal_status(env, status, contract_id, proposal_number);
}
