use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

use crate::models::agents_basic::basic_agent::BasicAgent;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub route: String,
    pub is_route_dynamic: bool,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

#[async_trait]
pub trait SpecialFunctions: Debug {
    // Use so that manager can get attributes from Agents 
    fn get_attributes_from_agent(&self) -> &BasicAgent;
    // This function will allow agents to execute their logic
    async fn execute(&mut self, fact_sheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
} 