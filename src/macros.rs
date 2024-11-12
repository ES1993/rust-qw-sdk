#[macro_export]
macro_rules! handle_request {
    ($request_builder: ident) => {{
        let failed_loop_num = 3;
        let mut index = 0;
        let mut res;

        while index < failed_loop_num {
            index += 1;
            if let Some(client) = $request_builder.try_clone() {
                let token = crate::inside::get_qw_inside_token().await?;
                res = client
                    .query(&[("access_token", &token)])
                    .send()
                    .await?
                    .json::<Value>()
                    .await?;

                dbg!(&res);
                if let Some(errcode) = res.get("errcode").and_then(|v| v.as_i64()) {
                    match errcode {
                        0 => return Ok(serde_json::from_value(res)?),
                        42001 => crate::inside::refresh_qw_inside_token(token.as_str()).await?,
                        _ => {
                            return Err(crate::error::QiWeError::ResponseError(
                                res["errmsg"].to_string(),
                            ))
                        }
                    }
                } else {
                    return Err(crate::error::QiWeError::CantHandleResponseData);
                }
            }
        }

        return Err(crate::error::QiWeError::TokenCantUse);
    }};
}
