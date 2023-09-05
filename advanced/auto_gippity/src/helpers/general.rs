use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use std::fs;

const CODE_TEMPLATE_PATH: &str =
    "/home/akane/Projects/personal/rust-playground/advanced/web_template/src/code_template.rs";
pub const WEB_SERVER_PROJECT_PATH: &str =
    "/home/akane/Projects/personal/rust-playground/advanced/web_template/";
const EXEC_MAIN_PATH: &str =
    "/home/akane/Projects/personal/rust-playground/advanced/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/home/akane/Projects/personal/rust-playground/advanced/auto_gippity/schemas/api_schema.json";

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Return success or try again
    match llm_response_res {
        Ok(llm_res) => llm_res,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice"),
    }
}

// Performs call to LLM GPT - Decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T =
        serde_json::from_str(&llm_response).expect("Failed to decode AI response from serde_json");

    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read executable main code")
}

// Save New Backend Code
pub fn save_backend_code(contents: &str) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write to main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &str) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

// To encourage specific output
pub fn extend_ai_function(ai_function: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_function(func_input);

    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the result of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_message: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_message.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: &str = "Build me a web server for making stock price API requests.";
        let agent_position: &str = "Managing Agent";
        let agent_operation: &str = "Defining user requirements";

        let res: String = ai_task_request(
            ai_func_param,
            agent_position,
            agent_operation,
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
