/*!

# 删除成员

最后更新：2021/08/09
请求方式：GET（HTTPS）
请求地址：https://qyapi.weixin.qq.com/cgi-bin/user/delete?access_token=ACCESS_TOKEN&userid=USERID

---

# 参数说明：

| 参数           | 必须 | 说明                     |
|----------------|------|--------------------------|
| access_token   | 是   | 调用接口凭证             |
| userid         | 是   | 成员UserID。对应管理端的账号 |

---

# 权限说明：

仅通讯录同步助手或第三方通讯录应用可调用。
若是绑定了腾讯企业邮，则会同时删除邮箱账号。

---

# 返回结果：

```json
{
   "errcode": 0,
   "errmsg": "deleted"
}
```

---

# 参数说明：

| 参数     | 说明                 |
|----------|----------------------|
| errcode  | 返回码               |
| errmsg   | 对返回码的文本描述内容 |

*/

use crate::{error::QiWeError, handle_request};

pub async fn run(userid: &str) -> Result<(), QiWeError> {
    let url = "https://qyapi.weixin.qq.com/cgi-bin/user/delete";
    let query = [("userid", userid)];
    let rb = reqwest::Client::new().get(url).query(&query);
    handle_request!(rb)
}
