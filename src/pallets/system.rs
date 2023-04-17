use crate::shared::*;
use crate::substrate::*;

pub fn system_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewAccount" => {
            let event = event.as_event::<polkadot::system::events::NewAccount>()?.unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index);
            Ok(())
        },
        "KilledAccount" => {
            let event = event.as_event::<polkadot::system::events::KilledAccount>()?.unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index);
            Ok(())
        },
        "Remarked" => {
            let event = event.as_event::<polkadot::system::events::Remarked>()?.unwrap();
            index_event_account_id(trees.clone(), event.sender, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
