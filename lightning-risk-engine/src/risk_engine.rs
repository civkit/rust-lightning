// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use crate::events::Event;

use bitcoin::secp256k1::PublicKey;

struct CounterpartyAccount {
	their_node_id: PublicKey,
}

pub struct RiskEngine {
	list_accounts: Vec<CounterpartyAccount>

	//TODO: add credentials config
}

impl RiskEngine {
	fn handle_open_channel_request(their_node_id: PublicKey, channel_request: events::OpenChannelRequest) -> bool {
		return true;
	}
}
