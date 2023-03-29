use clap::Parser;

use subxt::{
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
};

use sled::Tree;

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   pub url: String,
   /// Block number to start indexing from.
   #[arg(short, long)]
   pub block_height: u32,
}

#[derive(Clone)]
pub struct Trees {
    pub account_id: Tree,
    pub registrar_index: Tree,
}

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

    pub fn unserialize(vec: Vec<u8>) -> AccountIdKey {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

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

    pub fn unserialize(vec: Vec<u8>) -> RegistrarIndexKey {
        RegistrarIndexKey {
            registrar_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "pallet")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    Balances(Balances),
    #[serde(rename_all = "camelCase")]
    Identity(Identity),
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Free,
    Reserved,
}

use crate::polkadot::runtime_types::frame_support::traits::tokens::misc::BalanceStatus;

impl From<&BalanceStatus> for Status {
    fn from(x: &BalanceStatus) -> Status {
        match x {
            BalanceStatus::Free => Status::Free,
            BalanceStatus::Reserved => Status::Reserved,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum Balances {
    #[serde(rename_all = "camelCase")]
    Endowed {
        #[bincode(with_serde)]
        account: AccountId32,
        free_balance: u128,
    },
    #[serde(rename_all = "camelCase")]
    DustLost {
        #[bincode(with_serde)]
        account: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    Transfer {
        #[bincode(with_serde)]
        from: AccountId32,
        #[bincode(with_serde)]
        to: AccountId32,
        value: u128,
    },
    #[serde(rename_all = "camelCase")]
	BalanceSet {
        #[bincode(with_serde)]
	    who: AccountId32,
	    free: u128,
	    reserved: u128,
    },
    #[serde(rename_all = "camelCase")]
	Reserved {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Unreserved {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
    },
    #[serde(rename_all = "camelCase")]
	ReserveRepatriated {
        #[bincode(with_serde)]
		from: AccountId32,
        #[bincode(with_serde)]
		to: AccountId32,
		amount: u128,
		destination_status: Status,
	},
    #[serde(rename_all = "camelCase")]
	Deposit {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Withdraw {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Slashed {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Identity {
    #[serde(rename_all = "camelCase")]
    IdentitySet {
        #[bincode(with_serde)]
        who: AccountId32,
    },
    #[serde(rename_all = "camelCase")]
    IdentityCleared {
        #[bincode(with_serde)]
        who: AccountId32,
        deposit: u128,
    },
    #[serde(rename_all = "camelCase")]
    IdentityKilled {
        #[bincode(with_serde)]
        who: AccountId32,
        deposit: u128,
    },
    #[serde(rename_all = "camelCase")]
    JudgementRequested {
        #[bincode(with_serde)]
        who: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    JudgementUnrequested {
        #[bincode(with_serde)]
        who: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    JudgementGiven {
        #[bincode(with_serde)]
        target: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
	RegistrarAdded {
	    registrar_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityAdded {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityRemoved {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityRevoked {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
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



