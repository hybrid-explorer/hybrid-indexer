use crate::shared::*;
use crate::substrate::*;

pub fn transaction_payment_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "TransactionFeePaid" => {
            let event = event.as_event::<polkadot::transaction_payment::events::TransactionFeePaid>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
