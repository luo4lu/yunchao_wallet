use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ResponseBody<T> {
    pub fn new_success(data: Option<T>) -> Self {
        ResponseBody {
            code: 0,
            message: String::from("success"),
            data,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn return_unwrap_error(error: String) -> Self {
        ResponseBody {
            code: 60000,
            message: format!("处理信息错误:{}", error),
            data: None,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn object_status_error(error: String) -> Self {
        ResponseBody {
            code: 60001,
            message: format!("支付对象状态错误：{}", error),
            data: None,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn object_not_exit() -> Self {
        ResponseBody {
            code: 60002,
            message: format!("支付对象不存在"),
            data: None,
        }
    }
}