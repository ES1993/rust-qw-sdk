mod access_token;
mod error;

use error::QiWeInsideError;
use std::env;

#[derive(Debug)]
pub struct QiWeInside {
    ///每个企业都拥有唯一的corpid，获取此信息可在管理后台“我的企业”－“企业信息”下查看“企业ID”（需要有管理员权限）
    corpid: String,
    ///secret是企业应用里面用于保障数据安全的“钥匙”，每一个应用都有一个独立的访问密钥，为了保证数据的安全，secret务必不能泄漏。
    secret: String,
    ///access_token是企业后台去企业微信的后台获取信息时的重要票据，由corpid和secret产生。所有接口在通信时都需要携带此信息用于验证接口的访问权限
    access_token: String,
}

impl QiWeInside {
    pub async fn new() -> Result<Self, QiWeInsideError> {
        let corpid = env::var("QW_INSIDE_CORPID")
            .map_err(|_| QiWeInsideError::EnvError("QW_INSIDE_CORPID".to_string()))?;
        let secret = env::var("QW_INSIDE_SECRET")
            .map_err(|_| QiWeInsideError::EnvError("QW_INSIDE_SECRET".to_string()))?;
        let access_token = access_token::get(&corpid, &secret).await?;

        Ok(QiWeInside {
            corpid,
            secret,
            access_token,
        })
    }
}

#[tokio::test]
async fn test() {
    let res = QiWeInside::new().await;
    if let Err(e) = res {
        dbg!(e.to_string());
    }
}
