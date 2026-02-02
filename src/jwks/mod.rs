use crate::{
    cognito::{self, CognitoConfig},
    jwks::s3::{get_jwks_object, put_jwks_object},
};
use aws_sdk_s3::Client as S3Client;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

pub mod s3;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Jwk {
    pub alg: String,
    pub e: String,
    pub kid: String,
    pub kty: String,
    pub n: String,
    pub r#use: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

pub struct JwksResult {
    pub jwk: Jwk,
    pub jwks: Jwks,
}

async fn get_http_jwk(
    expected_kid: String,
    s3: &S3Client,
    http: &HttpClient,
    cognito: &CognitoConfig,
) -> JwksResult {
    let cognito_user_pool_id = &cognito.user_pool_id;
    let cognito_region = &cognito.region;
    let url =
        format!("https://cognito-idp.{cognito_region}.amazonaws.com/{cognito_user_pool_id}/.well-known/jwks.json");
    let body = http.get(url).send().await.unwrap().text().await.unwrap();
    let jwks: Jwks = serde_json::from_str(&body).unwrap();

    let jwk = match jwks.keys.iter().find(|&k| k.kid == expected_kid) {
        Some(value) => value.clone(),
        None => {
            panic!("No valid jwks key found! Maybe you are looking at the wrong cognito configuration?");
        }
    };

    put_jwks_object(s3, &jwks).await.unwrap();

    JwksResult { jwks, jwk }
}

async fn get_s3_jwk(
    expected_kid: String,
    http: &HttpClient,
    s3: &S3Client,
    cognito: &CognitoConfig,
) -> JwksResult {
    let jwks_from_s3 = get_jwks_object(s3).await;

    let out = match jwks_from_s3 {
        Ok(jwks) => match jwks.keys.iter().find(|&k| k.kid == expected_kid) {
            Some(value) => JwksResult {
                jwks: jwks.clone(),
                jwk: value.clone(),
            },
            None => get_http_jwk(expected_kid, s3, http, cognito).await,
        },
        Err(_) => get_http_jwk(expected_kid, s3, http, cognito).await,
    };

    out
}

pub async fn get_jwk(
    expected_kid: String,
    http: &HttpClient,
    s3: &S3Client,
    cognito: &CognitoConfig,
) -> JwksResult {
    let out = match &cognito.jwks {
        Some(jwks) => match jwks.keys.iter().find(|&k| k.kid == expected_kid) {
            Some(value) => JwksResult {
                jwks: jwks.clone(),
                jwk: value.clone(),
            },
            None => get_s3_jwk(expected_kid, http, s3, cognito).await,
        },
        None => get_s3_jwk(expected_kid, http, s3, cognito).await,
    };

    out
}
