use thiserror::Error;

#[derive(Error, Debug)]
pub enum QiWeError {
    #[error("无法获取企业微信接口Token")]
    CantGetToken,

    #[error("请求网络错误")]
    HttpError(#[from] reqwest::Error),

    #[error("无法获取环境变量:{0}")]
    EnvError(String),

    #[error("序列化json错误:{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("无法处理企业微信接口返回的数据格式")]
    CantHandleResponseData,

    #[error("企业微信接口错误:{0}")]
    ResponseError(String),

    #[error("无法初始化:{0}")]
    CantInit(String),

    #[error("企业微信Token无法使用")]
    TokenCantUse,
}
