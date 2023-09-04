use crate::models::agents_basic::basic_agent::{BasicAgent, AgentState};

use super::agent_traits::{FactSheet, ProjectScope};

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes  = BasicAgent {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![]
        };

        Self {
            attributes
        }
    }

    async fn call_project_scope(&mut self, fact_sheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", fact_sheet.project_description);
        
    }
}