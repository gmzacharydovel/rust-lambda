use crate::jwks::{get_jwk, Claims, Jwks, JwksResult};
use aws_sdk_s3::Client as S3Client;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lambda_http::{Error as UnknownError, Request};
use reqwest::Client as HttpClient;

pub(crate) struct AuthenticateReturnType {
    pub jwks: Jwks,
    pub claims: Claims,
}

pub async fn authenticate(
    _event: &Request,
    http: &HttpClient,
    s3: &S3Client,
    cognito: &CognitoConfig,
) -> Result<AuthenticateReturnType, UnknownError> {
    // clean this up
    let cognito_client_id = &cognito.client_id;
    let cognito_user_pool_id = &cognito.user_pool_id;
    let cognito_region = &cognito.region;
    let token = "eyJraWQiOiI0UDJZKzl3Q1pZZUZCWFJuZXJ1bGhNenVaQTlkcnVDbUZxMVk1XC9Jd0lIdz0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiI5NzA0NmFlOC03MDYxLTcwYTQtNDEwOC0zOGMxNDc0ZGY3MWYiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLmFwLW5vcnRoZWFzdC0xLmFtYXpvbmF3cy5jb21cL2FwLW5vcnRoZWFzdC0xX1FaMjVmM3JKeiIsImNvZ25pdG86dXNlcm5hbWUiOiI5NzA0NmFlOC03MDYxLTcwYTQtNDEwOC0zOGMxNDc0ZGY3MWYiLCJvcmlnaW5fanRpIjoiZjYzZGYzOTQtNTczNi00OTJiLTg3MmMtYzU4YzJkNzU0MGMyIiwiYXVkIjoiNGhzMDBzczY1bzV0ZWNsMjJnZ3JlMDRzODMiLCJldmVudF9pZCI6ImZhYmY2ZjJlLTBiZGItNDIxYy1iYTA4LWU0NTIwY2U0YTU2NyIsInRva2VuX3VzZSI6ImlkIiwiYXV0aF90aW1lIjoxNzY5NzEwMDAwLCJleHAiOjE3Njk3MTM2MDAsImlhdCI6MTc2OTcxMDAwMCwianRpIjoiZDhiNmZkMGMtOWM4ZS00ZWI5LTljNmQtOTNkOGU1OGI3Mjg5IiwiZW1haWwiOiJ6YWNoYXJ5LmRvdmVsQGdyYW5tYW5pYnVzLmNvbSJ9.yFDecNG7zyOnCHILjyLdbUxAHiu3nS5a9SmJh8vp5wmI7DTjtEdx9hVc9dFThExUoDoi0kAODzg7WzJ_D14GPVj5FprVoxTkcwcwHt8aK9VcVuMVjGqnXgtyxOeVu5vzpiEO80Q-bcFHrp7zQnag6Y0CaaILIY7oKd-jPGvc74k8tG0B9jiuk35IbyMMngEfCR-Ubg9d_HPztgCE0icEZ-EEalh2OQ2lhipKRh4FhSPkWkDzV5IVC9oqSMmoraw0jbDOG1k6aflVnVA2bAVbee_SiGkUeIeAxUjyIr6xfv-osVQrhiinklnuDbAJJkpxpePMsOIZJy7NpmKIK0TgMg";
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

pub(crate) trait JwksSetter {
    fn set_jwks(&mut self, jwks: Jwks) -> ();
}

impl JwksSetter for CognitoConfig {
    fn set_jwks(&mut self, jwks: Jwks) {
        self.jwks = Some(jwks);
    }
}
