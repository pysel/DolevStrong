use super::GenesisStrategy;
use crate::{communication::sync::wait_delta, consensus::ConsensusNode};

pub struct FollowerStrategy;

impl GenesisStrategy for FollowerStrategy {
    fn genesis_round(&self, self_node: &ConsensusNode) {
        wait_delta(); // Round zero: allow leader to send out a value proposal.

        if self_node.self_is_leader { panic!("leader node has follower's strategy") } // sanity check

        // let round_leader = 
    }
}