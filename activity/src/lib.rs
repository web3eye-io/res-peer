use linera_sdk::base::{ContractAbi, ServiceAbi};

pub struct ActivityAbi;

impl ContractAbi for ActivityAbi {
    type Parameters = ();
    type InitializationArgument = ();
    type Operation = ();
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for ActivityAbi {
    type Parameters = ();
    type Query = ();
    type QueryResponse = ();
}
