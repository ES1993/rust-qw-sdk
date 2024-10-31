/*!
获取access_token是调用企业微信API接口的第一步，
相当于创建了一个登录凭证，其它的业务API接口，
都需要依赖于access_token来鉴权调用者身份。
因此开发者，在使用业务接口前，
要明确access_token的颁发来源，使用正确的access_token。
请求方式： GET（HTTPS）
请求地址： https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid=ID&corpsecret=SECRET
corpid	是	企业ID，获取方式参考：术语说明-corpid
corpsecret	是	应用的凭证密钥，注意应用需要是启用状态，获取方式参考：术语说明-secret

权限说明：
每个应用有独立的secret，获取到的access_token只能本应用使用，所以每个应用的access_token应该分开来获取

返回结果：
{
   "errcode": 0,
   "errmsg": "ok",
   "access_token": "accesstoken000001",
   "expires_in": 7200
}

参数说明：
errcode	出错返回码，为0表示成功，非0表示调用失败
errmsg	返回码提示语
access_token	获取到的凭证，最长为512字节
expires_in	凭证的有效时间（秒）

注意事项：
开发者需要缓存access_token，用于后续接口的调用（注意：不能频繁调用gettoken接口，否则会受到频率拦截）。当access_token失效或过期时，需要重新获取。

access_token的有效期通过返回的expires_in来传达，正常情况下为7200秒（2小时）。
由于企业微信每个应用的access_token是彼此独立的，所以进行缓存时需要区分应用来进行存储。
access_token至少保留512字节的存储空间。
企业微信可能会出于运营需要，提前使access_token失效，开发者应实现access_token失效时重新获取的逻辑。
 */

use super::error::QiWeInsideError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Res {
    //出错返回码，为0表示成功，非0表示调用失败
    errcode: i32,

    //返回码提示语
    errmsg: String,

    //获取到的凭证，最长为512字节
    access_token: Option<String>,

    //凭证的有效时间（秒）
    expires_in: Option<i32>,
}

pub async fn get(corpid: &str, corpsecret: &str) -> Result<String, QiWeInsideError> {
    let client = reqwest::Client::new();

    let url = "https://qyapi.weixin.qq.com/cgi-bin/gettoken";
    let query = [("corpid", corpid), ("corpsecret", corpsecret)];

    let res = client
        .get(url)
        .query(&query)
        .send()
        .await?
        .json::<Res>()
        .await?;

    return res
        .access_token
        .ok_or(QiWeInsideError::CantGetToken(res.errmsg));
}
