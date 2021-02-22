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
            code: 70000,
            message: format!("处理信息错误:{}", error),
            data: None,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn object_status_error(error: String) -> Self {
        ResponseBody {
            code: 70001,
            message: format!("签约对象状态错误：{}", error),
            data: None,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn object_not_exit() -> Self {
        ResponseBody {
            code: 70002,
            message: format!("签约对象不存在"),
            data: None,
        }
    }
    pub fn return_none_error() -> Self {
        ResponseBody {
            code: 70003,
            message: format!("获取请求头为空"),
            data: None,
        }
    }
}