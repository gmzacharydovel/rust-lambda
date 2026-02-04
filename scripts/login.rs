use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let user_pool_id = env::var("AWS_COGNITO_USER_POOL_ID").unwrap();
    let client_id = env::var("AWS_COGNITO_CLIENT_ID").unwrap();
    let username = env::var("AWS_COGNITO_USERNAME").unwrap();
    let password = env::var("AWS_COGNITO_PASSWORD").unwrap();

    let mut command = Command::new("aws");
    command.args([
        "cognito-idp",
        "admin-initiate-auth",
        "--user-pool-id",
        &user_pool_id,
        "--client-id",
        &client_id,
        "--auth-flow",
        "ADMIN_USER_PASSWORD_AUTH",
        "--auth-parameters",
        &format!("USERNAME=\"{username}\",PASSWORD=\"{password}\""),
    ]);
    let content = command.output().unwrap();
    let response = String::from_utf8(content.stdout).unwrap();
    let mut file = File::create(".authentication.json").unwrap();

    file.write_all(response.as_bytes()).unwrap();
}
