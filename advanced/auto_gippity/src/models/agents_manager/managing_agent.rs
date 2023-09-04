use crate::models::{agents_basic::basic_agent::BasicAgent, agents::agent_traits::{FactSheet, SpecialFunctions}};

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    fact_sheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>
}