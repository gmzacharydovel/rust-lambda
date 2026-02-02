use crate::cognito::CognitoConfig;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use http_handler::{function_handler, FunctionHandlerReturn, State};
use lambda_http::{run, service_fn, tracing, Error};
use lambda_http::{Body, Error as UnknownError, Response as HttpResponse};
use reqwest::Client as HttpClient;
use std::env;
use tokio::sync::RwLock;

mod cognito;
mod http_handler;
mod jwks;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let s3 = &S3Client::new(&shared_config);
    let http = &HttpClient::new();
    let lock = RwLock::new(State {
        s3,
        http,
        cognito: CognitoConfig {
            user_pool_id: env::var("AWS_COGNITO_USER_POOL_ID")?,
            client_id: env::var("AWS_COGNITO_CLIENT_ID")?,
            region: env::var("AWS_COGNITO_REGION")?,
            jwks: None,
        },
    });

    run(service_fn(
        async |e| -> Result<HttpResponse<Body>, UnknownError> {
            let state = lock.read().await;
            let FunctionHandlerReturn {
                jwks: current_jwks,
                result,
            } = function_handler(e, &state).await?;

            let state = lock.read().await;

            match state.cognito.jwks {
                Some(ref previous_jwks) if *previous_jwks == current_jwks => {
                    // Do nothing
                }
                _ => {
                    let mut state = lock.write().await;
                    state.cognito.jwks = Some(current_jwks);
                }
            }

            result
        },
    ))
    .await
}
