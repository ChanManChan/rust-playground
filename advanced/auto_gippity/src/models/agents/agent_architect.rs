use std::time::Duration;

use crate::{
    models::agents_basic::{basic_agent::{BasicAgent, AgentState}, basic_traits::BasicTraits}, 
    helpers::{general::{ai_task_request_decoded, check_status_code}, command_line::PrintCommand}, ai_functions::aifunc_architect::print_project_scope, ai_functions::aifunc_architect::print_site_urls
};
use async_trait::async_trait;
use reqwest::Client;
use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

// Solutions Architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes: BasicAgent  = BasicAgent {
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
        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            &msg_context, 
            &self.attributes.position, 
            get_function_string!(print_project_scope), 
            print_project_scope
        ).await;

        fact_sheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }

    async fn call_determine_external_urls(&mut self, fact_sheet: &mut FactSheet, msg_context: String) {
        let ai_response = ai_task_request_decoded::<Vec<String>>(
            &msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls
        ).await;

        fact_sheet.external_urls = Some(ai_response);
        self.attributes.update_state(AgentState::UnitTesting);
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) ->  &BasicAgent {
        &self.attributes
    }

    async fn execute(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {
        
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope: ProjectScope = self.call_project_scope(fact_sheet).await;
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(fact_sheet, fact_sheet.project_description.clone()).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                },
                AgentState::UnitTesting => {
                    // let mut exclude_urls: Vec<String> = vec![];
                    let mut correct_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Defining urls to check
                    let urls: &Vec<String> = fact_sheet
                        .external_urls.as_ref().expect("No URL object on Fact Sheet");
                    
                    // Find faulty urls
                    for url in urls {
                        let endpoint_str: String = format!("Testing URL endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(&self.attributes.position, &endpoint_str);
                        
                        // Perform URL Test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code == 200 {
                                    correct_urls.push(url.clone());
                                }
                                // if status_code != 200 {
                                //     exclude_urls.push(url.clone())
                                // }
                            }
                            Err(e) => println!("Error checking {}: {}", url, e)
                        }
                    }
                        
                    if correct_urls.len() > 0 {
                        // let new_urls: Vec<String> = fact_sheet
                        //     .external_urls
                        //     .as_ref()
                        //     .unwrap()
                        //     .iter()
                        //     .filter(|url| !exclude_urls.contains(&url))
                        //     .cloned()
                        //     .collect();

                        fact_sheet.external_urls = Some(correct_urls);
                    }

                    self.attributes.state = AgentState::Finished;
                },
                _ => {
                    self.attributes.state = AgentState::Finished
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut fact_sheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None
        };

        agent.execute(&mut fact_sheet).await.expect("Unable to execute Solutions Architect Agent");

        assert!(fact_sheet.project_scope != None);
        assert!(fact_sheet.external_urls.is_some());

        dbg!(fact_sheet);
    }
}