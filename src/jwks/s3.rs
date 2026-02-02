use crate::jwks::Jwks;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::{
    get_object::GetObjectError,
    put_object::{PutObjectError, PutObjectOutput},
};
use aws_sdk_s3::primitives::ByteStream;
use aws_smithy_types::body::SdkBody;

#[allow(dead_code)]
#[derive(Debug)]
pub enum PutJwksObjectError {
    PutObjectError(Box<SdkError<PutObjectError>>),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for PutJwksObjectError {
    fn from(err: serde_json::Error) -> PutJwksObjectError {
        PutJwksObjectError::Json(err)
    }
}

impl From<SdkError<PutObjectError>> for PutJwksObjectError {
    fn from(err: SdkError<PutObjectError>) -> PutJwksObjectError {
        PutJwksObjectError::PutObjectError(Box::new(err))
    }
}

pub async fn put_jwks_object(
    client: &aws_sdk_s3::Client,
    data: &Jwks,
) -> Result<PutObjectOutput, PutJwksObjectError> {
    let as_string = serde_json::to_string(data)?;
    let stream = ByteStream::new(SdkBody::from(as_string));
    let object = client
        .put_object()
        .bucket("jwks")
        .key("jwks.json")
        .body(stream)
        .send()
        .await?;

    Ok(object)
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum GetJwksObjectError {
    GetObjectError(Box<SdkError<GetObjectError>>),
    ByteStreamError(aws_sdk_s3::primitives::ByteStreamError),
    Utf8Error(std::str::Utf8Error),
    Json(serde_json::Error),
}

impl From<SdkError<GetObjectError>> for GetJwksObjectError {
    fn from(err: SdkError<GetObjectError>) -> GetJwksObjectError {
        GetJwksObjectError::GetObjectError(Box::new(err))
    }
}

impl From<aws_sdk_s3::primitives::ByteStreamError> for GetJwksObjectError {
    fn from(err: aws_sdk_s3::primitives::ByteStreamError) -> GetJwksObjectError {
        GetJwksObjectError::ByteStreamError(err)
    }
}

impl From<std::str::Utf8Error> for GetJwksObjectError {
    fn from(err: std::str::Utf8Error) -> GetJwksObjectError {
        GetJwksObjectError::Utf8Error(err)
    }
}

impl From<serde_json::Error> for GetJwksObjectError {
    fn from(err: serde_json::Error) -> GetJwksObjectError {
        GetJwksObjectError::Json(err)
    }
}

pub async fn get_jwks_object(client: &aws_sdk_s3::Client) -> Result<Jwks, GetJwksObjectError> {
    let object = client
        .get_object()
        .bucket("jwks")
        .key("jwks.json")
        .send()
        .await?;
    let body = object.body.collect().await?;
    let data_bytes = body.into_bytes();
    let data_string = str::from_utf8(&data_bytes)?;
    let jwks: Jwks = serde_json::from_str(data_string)?;

    Ok(jwks)
}
