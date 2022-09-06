use serde::{Deserialize, Serialize};
/// A struct representing the client's details in our system.
#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    /// A unique identifier to identify the id of the client.
    id: u16,
    /// Funds avaiable for the client to buy assets.
    available: f64,
    /// Funds that are held due to dispute in certain transactions.
    held: f64,
    /// Total funds in the client's account. This is summaiton of held and available funds.
    total: f64,
}
