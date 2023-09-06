#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

use crate::models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let user_request: String = get_user_response("What web server are we building today?");
    let mut manager_agent = ManagingAgent::new(user_request.as_str())
        .await
        .expect("Error creating managing agent");

    manager_agent.execute_project().await;

    dbg!(manager_agent);
}
