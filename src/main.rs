#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{ stringify!($func) }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::{ManagingAgent};

#[tokio::main]
async fn main() {

    // let user_req: String = get_user_response("What webserver are we building today ?");
    let user_req: String = get_user_response("What website are we building today ?"); // 124.
    // ANSWER: Need a website that provides forex prices for a given list of forex pairs. Do not use some strange library that i never heard of.
    
    let mut managing_agent: ManagingAgent = ManagingAgent::new(user_req) // mutable because call execute_project
    .await
    .expect("Unable to create Managing Agent");

    managing_agent.execute_project().await;
    // dbg!(managing_agent);
}
