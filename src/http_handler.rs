use crate::cognito::{authenticate, CognitoConfig};
use crate::jwks::Jwks;
use aws_sdk_s3::Client as S3Client;
use lambda_http::{Body, Error as UnknownError, Request, RequestExt, Response as HttpResponse};
use reqwest::Client as HttpClient;
use std::cell::RefCell;
use std::sync::RwLock;

pub(crate) struct State<'a> {
    pub s3: &'a S3Client,
    pub http: &'a HttpClient,
    pub cognito: CognitoConfig,
}

impl<'a> Clone for State<'a> {
    fn clone(&self) -> Self {
        State {
            s3: self.s3,
            http: self.http,
            cognito: self.cognito.clone(),
        }
    }
}

pub(crate) struct FunctionHandlerReturn {
    pub jwks: Jwks,
    pub result: Result<HttpResponse<Body>, UnknownError>,
}

// https://cognito-idp.<Region>.amazonaws.com/<userPoolId>/.well-known/jwks.json
pub(crate) async fn function_handler<'a>(
    event: Request,
    state: &State<'a>,
) -> Result<FunctionHandlerReturn, UnknownError> {
    let auth = authenticate(&event, &state.http, &state.s3, &state.cognito).await?;

    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    let resp = HttpResponse::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::Text(message))
        .map_err(Box::new)?;

    Ok(FunctionHandlerReturn {
        result: Ok(resp),
        jwks: auth.jwks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();
        let client = reqwest::Client::new();
        let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let s3 = aws_sdk_s3::Client::new(&shared_config);

        //let response = function_handler(request, &client, &s3).await.unwrap();
        //assert_eq!(response.status(), 200);

        //let body_bytes = response.body().to_vec();
        //let body_string = String::from_utf8(body_bytes).unwrap();

        //assert_eq!(
        //    body_string,
        //    "Hello world, this is an AWS Lambda HTTP request"
        //);
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "new-lambda-project".into());

        let client = reqwest::Client::new();
        let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let s3 = aws_sdk_s3::Client::new(&shared_config);

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        //let response = function_handler(request, &client, &s3).await.unwrap();
        //assert_eq!(response.status(), 200);

        //let body_bytes = response.body().to_vec();
        //let body_string = String::from_utf8(body_bytes).unwrap();

        //assert_eq!(
        //    body_string,
        //    "Hello new-lambda-project, this is an AWS Lambda HTTP request"
        //);
    }
}
