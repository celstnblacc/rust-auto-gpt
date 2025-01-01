use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::{Message};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

pub const DEBUG_MODE : bool = true;

const CODE_TEMPLATE_PATH: &str =
    "/Users/macair15/Documents/DevOpsCelstn/RustAutoGPT_UDEMY/web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str =
    "/Users/macair15/Documents/DevOpsCelstn/RustAutoGPT_UDEMY/web_template/src/main.rs";
const API_SCHEMA_PATH: &str =
    "/Users/macair15/Documents/DevOpsCelstn/RustAutoGPT_UDEMY/rust-auto-gpt/schemas/api_schema.json";
pub const WEB_SERVER_PROJECT_PATH: &str =
    "/Users/macair15/Documents/DevOpsCelstn/RustAutoGPT_UDEMY/web_template";


pub fn extend_ai_function(ai_funct: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_funct(func_input);
    //    dbg!(ai_function_str);

    // Extend the string to encourage only printing the ouput
    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTIONS: You are a function printer. You ONLY print the rwsults of functions.
    Nothing else. No commentary. Here is the input to the function {}.
    Print out what the function will return.",
        ai_function_str, func_input
    );

    // Return meassage
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Perform call to llm GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    // for <'a>: For lifetime a
    //fn(&str): function passed
    //-> &'static str: return lifetime static string
    function_passed: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_ai_function(function_passed, &msg_context);
    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Call GPT response
    // Clone to not dereference it
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Return success or try again or fail
    match llm_response_res {
        Ok(res) => res,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call GPT"),
    }
}

// Perform call to llm GPT - decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    // for <'a>: For lifetime a
    //fn(&str): function passed
    //-> &'static str: return lifetime static string
    function_passed: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String = ai_task_request(
        msg_context,
        agent_position,
        agent_operation,
        function_passed,
    )
    .await;

    // Decode the response
    let decoded_response: T = serde_json::from_str(&llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");
    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    std::fs::read_to_string(path.clone()).unwrap_or_else(|_| {
        panic!("The file at '{}' is missing. Please create the file and try again.", path)
    })
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Save new backend code
pub fn save_backend_code(contents: &String) {
    // Clean up markdown markers
    let cleaned_contents = contents
        .trim_start_matches("```rust")
        .trim_start_matches("```")
        .trim_end_matches("```");

    // Write to the file
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, cleaned_contents).expect("Failed to save backend code");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        // let x_str = convert_user_input_to_goal("dummy input");
        let extended_msg: Message = extend_ai_function(convert_user_input_to_goal, "dummy input");
        // dbg!(&extended_msg);
        assert!(
            extended_msg.role == "system".to_string(),
            "The role should be system"
        );
    }

    // Cost money to run the test with tokio call OpenAI
    #[tokio::test]
    async fn test_ai_task_request() {
        let func_param = "Build me a webserver for making stock price api requests.".to_string();
        let res = ai_task_request(
            func_param,
            "Manging agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;
        dbg!(&res);
        assert!(res.len() > 20, "Response should not be empty");
    }
}
