use std::env::VarError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum QiWeInsideError {
    #[error("rust-qw-sdk:无法获取access_token:{0}")]
    CantGetToken(String),

    #[error("rust-qw-sdk:请求网络错误")]
    HttpError(#[from] reqwest::Error),

    #[error("rust-qw-sdk:无法获取环境变量:{0}")]
    EnvError(String),
}
