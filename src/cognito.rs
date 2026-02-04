use crate::jwks::{get_jwk, Claims, Jwks, JwksResult};
use aws_sdk_s3::Client as S3Client;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lambda_http::{Error as UnknownError, Request};
use reqwest::Client as HttpClient;

#[allow(dead_code)]
pub(crate) struct AuthenticateReturnType {
    pub jwks: Jwks,
    pub claims: Claims,
}

pub async fn authenticate(
    event: &Request,
    http: &HttpClient,
    s3: &S3Client,
    cognito: &CognitoConfig,
) -> Result<AuthenticateReturnType, UnknownError> {
    // clean this up
    let cognito_client_id = &cognito.client_id;
    let cognito_user_pool_id = &cognito.user_pool_id;
    let cognito_region = &cognito.region;

    let token = &event
        .headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()["Bearer ".len()..];
    print!("token {}", token);
    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.unwrap();
    let JwksResult {
        jwks,
        jwk: matching_jwk,
    } = get_jwk(kid, http, s3, cognito).await;
    let decoding_key = DecodingKey::from_rsa_components(&matching_jwk.n, &matching_jwk.e)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[cognito_client_id]);
    let issuer =
        format!("https://cognito-idp.{cognito_region}.amazonaws.com/{cognito_user_pool_id}");
    validation.set_issuer(&[issuer]);

    let result = jsonwebtoken::decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(AuthenticateReturnType {
        jwks,
        claims: result.claims,
    })
}

#[derive(Clone)]
pub(crate) struct CognitoConfig {
    pub client_id: String,
    pub user_pool_id: String,
    pub region: String,
    pub jwks: Option<Jwks>,
}
