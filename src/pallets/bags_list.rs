use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn bags_list_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Rebagged" => {
            let event = event
                .as_event::<polkadot::voter_list::events::Rebagged>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        "ScoreUpdated" => {
            let event = event
                .as_event::<polkadot::voter_list::events::ScoreUpdated>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
