/*!
# 读取成员

最后更新：2024/07/24
应用只能获取可见范围内的成员信息，且每种应用获取的字段有所不同，在返回结果说明中会逐个说明。企业通讯录安全特别重要，企业微信将持续升级加固通讯录接口的安全机制，以下是关键的变更点：

从2022年6月20号20点开始，除通讯录同步以外的基础应用（如客户联系、微信客服、会话存档、日程等），以及新创建的自建应用与代开发应用，调用该接口时，不再返回以下字段：头像、性别、手机、邮箱、企业邮箱、员工个人二维码、地址，应用需要通过oauth2手工授权的方式获取管理员与员工本人授权的字段。

【重要】从2022年8月15日10点开始，“企业管理后台 - 管理工具 - 通讯录同步”的新增IP将不能再调用此接口，企业可通过「获取成员ID列表」和「获取部门ID列表」接口获取userid和部门ID列表。查看调整详情。

---

# 请求方式：GET（HTTPS）

请求地址：https://qyapi.weixin.qq.com/cgi-bin/user/get?access_token=ACCESS_TOKEN&userid=USERID

---

# 参数说明：

| 参数           | 必须 | 说明                                                                                 |
|----------------|------|--------------------------------------------------------------------------------------|
| access_token   | 是   | 调用接口凭证                                                                         |
| userid         | 是   | 成员UserID。对应管理端的账号，企业内必须唯一。不区分大小写，长度为1~64个字节        |

---

# 权限说明：

应用须拥有指定成员的查看权限。

---

# 返回结果：

```json
{
    "errcode": 0,
    "errmsg": "ok",
    "userid": "zhangsan",
    "name": "张三",
    "department": [1, 2],
    "order": [1, 2],
    "position": "后台工程师",
    "mobile": "13800000000",
    "gender": "1",
    "email": "zhangsan@gzdev.com",
    "biz_mail":"zhangsan@qyycs2.wecom.work",
    "is_leader_in_dept": [1, 0],
    "direct_leader":["lisi"],
    "avatar": "http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/0",
    "thumb_avatar": "http://wx.qlogo.cn/mmopen/ajNVdqHZLLA3WJ6DSZUfiakYe37PKnQhBIeOQBO4czqrnZDS79FH5Wm5m4X69TBicnHFlhiafvDwklOpZeXYQQ2icg/100",
    "telephone": "020-123456",
    "alias": "jackzhang",
    "address": "广州市海珠区新港中路",
    "open_userid": "xxxxxx",
    "main_department": 1,
    "extattr": {
        "attrs": [
            {
                "type": 0,
                "name": "文本名称",
                "text": {
                    "value": "文本"
                }
            },
            {
                "type": 1,
                "name": "网页名称",
                "web": {
                    "url": "http://www.test.com",
                    "title": "标题"
                }
            }
        ]
    },
    "status": 1,
    "qr_code": "https://open.work.weixin.qq.com/wwopen/userQRCode?vcode=xxx",
    "external_position": "产品经理",
    "external_profile": {
        "external_corp_name": "企业简称",
        "wechat_channels": {
            "nickname": "视频号名称",
            "status": 1
        },
        "external_attr": [{
                "type": 0,
                "name": "文本名称",
                "text": {
                    "value": "文本"
                }
            },
            {
                "type": 1,
                "name": "网页名称",
                "web": {
                    "url": "http://www.test.com",
                    "title": "标题"
                }
            },
            {
                "type": 2,
                "name": "测试app",
                "miniprogram": {
                    "appid": "wx8bd80126147dFAKE",
                    "pagepath": "/index",
                    "title": "my miniprogram"
                }
            }
        ]
    }
}
```

---

# 应用获取敏感字段的说明

为保护企业数据与用户隐私，从6月20号20点开始，新创建的自建应用与代开发应用，调用该接口时，不再返回以下字段：头像、性别、手机、邮箱、企业邮箱、员工个人二维码、地址，应用需要通过oauth2手工授权的方式获取管理员与员工本人授权的字段。

*/

use crate::{error::QiWeError, handle_request};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// 成员UserID。对应管理端的账号，企业内必须唯一。不区分大小写，长度为1~64个字节；第三方应用返回的值为open_userid
    pub userid: Option<String>,

    /// 成员名称；第三方不可获取，调用时返回userid以代替name；代开发自建应用需要管理员授权才返回；对于非第三方创建的成员，第三方通讯录应用也不可获取；未返回name的情况需要通过通讯录展示组件来展示名字
    pub name: Option<String>,

    /// 手机号码，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub mobile: Option<String>,

    /// 成员所属部门id列表，仅返回该应用有查看权限的部门id；成员授权模式下，固定返回根部门id，即固定为1。对授权了“组织架构信息”权限的第三方应用或授权了“组织架构信息”-“部门及父部门ID、部门负责人”权限的代开发应用，返回成员所属的全部部门id
    pub department: Option<Vec<i32>>,

    /// 部门内的排序值，默认为0。数量必须和department一致，数值越大排序越前面。值范围是[0, 2^32)。成员授权模式下不返回该字段
    pub order: Option<Vec<i32>>,

    /// 职务信息；代开发自建应用需要管理员授权才返回；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub position: Option<String>,

    /// 性别。0表示未定义，1表示男性，2表示女性。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段。注：不可获取指返回值0
    pub gender: Option<String>,

    /// 邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub email: Option<String>,

    /// 企业邮箱，代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub biz_mail: Option<String>,

    /// 表示在所在的部门内是否为部门负责人，数量与department一致；第三方通讯录应用或者授权了“组织架构信息-应用可获取企业的部门组织架构信息-部门负责人”权限的第三方应用和代开发应用可获取；对于非第三方创建的成员，第三方通讯录应用不可获取；上游企业不可获取下游企业成员该字段
    pub is_leader_in_dept: Option<Vec<i32>>,

    /// 直属上级UserID，返回在应用可见范围内的直属上级列表，最多有1个直属上级；第三方通讯录应用或者授权了“组织架构信息-应用可获取可见范围内成员组织架构信息-直属上级”权限的第三方应用和代开发应用可获取；对于非第三方创建的成员，第三方通讯录应用不可获取；上游企业不可获取下游企业成员该字段
    pub direct_leader: Option<Vec<String>>,

    /// 头像url。 代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub avatar: Option<String>,

    /// 头像缩略图url。第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub thumb_avatar: Option<String>,

    /// 座机。代开发自建应用需要管理员授权才返回；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub telephone: Option<String>,

    /// 别名；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub alias: Option<String>,

    /// 扩展属性，字段详见成员扩展属性。代开发自建应用需要管理员授权才返回；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub extattr: Option<Value>,

    /// 激活状态: 1=已激活，2=已禁用，4=未激活，5=退出企业。已激活代表已激活企业微信或已关注微信插件（原企业号）。未激活代表既未激活企业微信又未关注微信插件（原企业号）。
    pub status: Option<i32>,

    /// 员工个人二维码，扫描可添加为外部联系人(注意返回的是一个url，可在浏览器上打开该url以展示二维码)；代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub qr_code: Option<String>,

    /// 成员对外属性，字段详情见对外属性；代开发自建应用需要管理员授权才返回；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub external_profile: Option<Value>,

    /// 对外职务，如果设置了该值，则以此作为对外展示的职务，否则以position来展示。代开发自建应用需要管理员授权才返回；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub external_position: Option<String>,

    /// 地址。代开发自建应用需要管理员授权且成员oauth2授权获取；第三方仅通讯录应用可获取；对于非第三方创建的成员，第三方通讯录应用也不可获取；上游企业不可获取下游企业成员该字段
    pub address: Option<String>,

    /// 全局唯一。对于同一个服务商，不同应用获取到企业内同一个成员的open_userid是相同的，最多64个字节。仅第三方应用可获取。
    pub open_userid: Option<String>,

    /// 主部门，仅当应用对主部门有查看权限时返回。
    pub main_department: Option<i32>,
}

pub async fn run(userid: &str) -> Result<Response, QiWeError> {
    let url = "https://qyapi.weixin.qq.com/cgi-bin/user/get";
    let query = [("userid", userid)];
    let rb = reqwest::Client::new().get(url).query(&query);
    handle_request!(rb)
}
