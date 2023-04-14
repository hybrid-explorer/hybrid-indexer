use clap::Parser;

use subxt::{
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
};

use sled::Tree;

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct ParaId(pub u32);

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

use crate::pallets::bags_list::BagsList;
use crate::pallets::balances::Balances;
use crate::pallets::bounties::Bounties;
use crate::pallets::child_bounties::ChildBounties;
use crate::pallets::claims::Claims;
use crate::pallets::collective::Collective;
use crate::pallets::democracy::Democracy;
use crate::pallets::election_provider_multi_phase::ElectionProviderMultiPhase;
use crate::pallets::elections_phragmen::PhragmenElection;
use crate::pallets::fast_unstake::FastUnstake;
use crate::pallets::identity::Identity;
use crate::pallets::indices::Indices;
use crate::pallets::multisig::Multisig;
use crate::pallets::nomination_pools::NominationPools;
use crate::pallets::proxy::Proxy;
use crate::pallets::system::System;
use crate::pallets::tips::Tips;
use crate::pallets::transaction_payment::TransactionPayment;
use crate::pallets::treasury::Treasury;
use crate::pallets::vesting::Vesting;

use crate::pallets::polkadot::auctions::Auctions;
use crate::pallets::polkadot::crowdloan::Crowdloan;
use crate::pallets::polkadot::parachains_disputes::ParasDisputes;
use crate::pallets::polkadot::parachains_hrmp::Hrmp;
use crate::pallets::polkadot::parachains_paras::Paras;
use crate::pallets::polkadot::parachains_ump::Ump;
use crate::pallets::polkadot::paras_registrar::Registrar;
use crate::pallets::polkadot::slots::Slots;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   pub url: Option<String>,
   /// Block number to start indexing from.
   #[arg(short, long)]
   pub block_height: Option<u32>,
}

#[derive(Clone)]
pub struct Trees {
    pub root: sled::Db,
    pub account_id: Tree,
    pub account_index: Tree,
    pub auction_index: Tree,
    pub bounty_index: Tree,
    pub candidate_hash: Tree,
    pub message_id: Tree,
    pub para_id: Tree,
    pub pool_id: Tree,
    pub proposal_hash: Tree,
    pub proposal_index: Tree,
    pub ref_index: Tree,
    pub registrar_index: Tree,
    pub tip_hash: Tree,
}

#[derive(PartialEq, Debug)]
pub struct AccountIdKey {
    pub account_id: <SubstrateConfig as Config>::AccountId,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl AccountIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_id.0.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AccountIndexKey {
    pub account_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl AccountIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        AccountIndexKey {
            account_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AuctionIndexKey {
    pub auction_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl AuctionIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.auction_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        AuctionIndexKey {
            auction_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BountyIndexKey {
    pub bounty_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl BountyIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.bounty_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        BountyIndexKey {
            bounty_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct CandidateHashKey {
    pub candidate_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl CandidateHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.candidate_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        CandidateHashKey {
            candidate_hash: vector_as_u8_32_array(&vec[0..32].to_vec()),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct MessageIdKey {
    pub message_id: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl MessageIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.message_id.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        MessageIdKey {
            message_id: vector_as_u8_32_array(&vec[0..32].to_vec()),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ParaIdKey {
    pub para_id: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl ParaIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.para_id.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        ParaIdKey {
            para_id: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PoolIdKey {
    pub pool_id: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl PoolIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.pool_id.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        PoolIdKey {
            pool_id: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RefIndexKey {
    pub ref_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl RefIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.ref_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        RefIndexKey {
            ref_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct RegistrarIndexKey {
    pub registrar_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl RegistrarIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.registrar_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        RegistrarIndexKey {
            registrar_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ProposalHashKey {
    pub proposal_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl ProposalHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.proposal_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        ProposalHashKey {
            proposal_hash: vector_as_u8_32_array(&vec[0..32].to_vec()),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ProposalIndexKey {
    pub proposal_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl ProposalIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.proposal_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        ProposalIndexKey {
            proposal_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TipHashKey {
    pub tip_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl TipHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.tip_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        TipHashKey {
            tip_hash: vector_as_u8_32_array(&vec[0..32].to_vec()),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "pallet")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    Auctions(Auctions),
    #[serde(rename_all = "camelCase")]
    BagsList(BagsList),
    #[serde(rename_all = "camelCase")]
    Balances(Balances),
    #[serde(rename_all = "camelCase")]
    Bounties(Bounties),
    #[serde(rename_all = "camelCase")]
    ChildBounties(ChildBounties),
    #[serde(rename_all = "camelCase")]
    Claims(Claims),
    #[serde(rename_all = "camelCase")]
    Collective(Collective),
    #[serde(rename_all = "camelCase")]
    Crowdloan(Crowdloan),
    #[serde(rename_all = "camelCase")]
    Democracy(Democracy),
    #[serde(rename_all = "camelCase")]
    ElectionProviderMultiPhase(ElectionProviderMultiPhase),
    #[serde(rename_all = "camelCase")]
    FastUnstake(FastUnstake),
    #[serde(rename_all = "camelCase")]
    Hrmp(Hrmp),
    #[serde(rename_all = "camelCase")]
    Paras(Paras),
    #[serde(rename_all = "camelCase")]
    ParasDisputes(ParasDisputes),
    #[serde(rename_all = "camelCase")]
    PhragmenElection(PhragmenElection),
    #[serde(rename_all = "camelCase")]
    Identity(Identity),
    #[serde(rename_all = "camelCase")]
    Indices(Indices),
    #[serde(rename_all = "camelCase")]
    Multisig(Multisig),
    #[serde(rename_all = "camelCase")]
    NominationPools(NominationPools),
    #[serde(rename_all = "camelCase")]
    Proxy(Proxy),
    #[serde(rename_all = "camelCase")]
    Registrar(Registrar),
    #[serde(rename_all = "camelCase")]
    Slots(Slots),
    #[serde(rename_all = "camelCase")]
    System(System),
    #[serde(rename_all = "camelCase")]
    Tips(Tips),
    #[serde(rename_all = "camelCase")]
    TransactionPayment(TransactionPayment),
    #[serde(rename_all = "camelCase")]
    Treasury(Treasury),
    #[serde(rename_all = "camelCase")]
    Ump(Ump),
    #[serde(rename_all = "camelCase")]
    Vesting(Vesting),
}

pub fn vector_as_u8_32_array(vector: &Vec<u8>) -> [u8; 32] {
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = vector[i];
    }
    arr
}

pub fn vector_as_u8_4_array(vector: &Vec<u8>) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for i in 0..4 {
        arr[i] = vector[i];
    }
    arr
}



