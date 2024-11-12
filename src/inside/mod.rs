/*!
# 企业内部应用接口
*/

pub mod contacts;
mod token;

use crate::error::QiWeError;
use std::{env, sync::Arc};
use tokio::sync::{Mutex, OnceCell};

static QI_WE_INSIDE: OnceCell<Result<Arc<Mutex<QiWeInside>>, QiWeError>> = OnceCell::const_new();

async fn get_qw_inside() -> Result<Arc<Mutex<QiWeInside>>, QiWeError> {
    QI_WE_INSIDE
        .get_or_init(|| async { QiWeInside::new().await.map(|r| Arc::new(Mutex::new(r))) })
        .await
        .as_ref()
        .map(Arc::clone)
        .map_err(|e| QiWeError::CantInit(e.to_string()))
}

async fn get_qw_inside_token() -> Result<String, QiWeError> {
    let qw = get_qw_inside().await?;
    let qw = qw.lock().await;
    Ok(qw.get_token().await)
}

async fn refresh_qw_inside_token(token: &str) -> Result<(), QiWeError> {
    let qw = get_qw_inside().await?;
    let mut qw = qw.lock().await;
    qw.refresh_token(token).await
}

#[derive(Debug, Default, Clone)]
struct QiWeInside {
    ///每个企业都拥有唯一的corpid，获取此信息可在管理后台“我的企业”－“企业信息”下查看“企业ID”（需要有管理员权限）
    corpid: String,

    ///secret是企业应用里面用于保障数据安全的“钥匙”，每一个应用都有一个独立的访问密钥，为了保证数据的安全，secret务必不能泄漏。
    secret: String,

    ///access_token是企业后台去企业微信的后台获取信息时的重要票据，由corpid和secret产生。所有接口在通信时都需要携带此信息用于验证接口的访问权限
    token: String,
}

impl QiWeInside {
    pub async fn new() -> Result<Self, QiWeError> {
        let corpid = env::var("QW_INSIDE_CORPID")
            .map_err(|_| QiWeError::EnvError("QW_INSIDE_CORPID".to_string()))?;
        let secret = env::var("QW_INSIDE_SECRET")
            .map_err(|_| QiWeError::EnvError("QW_INSIDE_SECRET".to_string()))?;

        let token = token::run(&corpid, &secret).await?;
        if let Some(token) = token.access_token {
            let mut qw: QiWeInside = QiWeInside {
                corpid,
                secret,
                ..Default::default()
            };
            qw.token = token;
            return Ok(qw);
        } else {
            return Err(QiWeError::CantGetToken);
        }
    }

    pub async fn get_token(&self) -> String {
        return self.token.clone();
    }

    pub async fn refresh_token(&mut self, token: &str) -> Result<(), QiWeError> {
        if self.token == token {
            let token = token::run(&self.corpid, &self.secret).await?;
            if let Some(token) = token.access_token {
                self.token = token;
            } else {
                return Err(QiWeError::CantGetToken);
            }
        }
        Ok(())
    }
}
