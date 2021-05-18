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
            code: 90000,
            message: format!("数据信息错误:{}", error),
            data: None,
        }
    }
    pub fn url_is_none() -> Self {
        ResponseBody {
            code: 90001,
            message: format!("url is none,please input url"),
            data: None,
        }
    }

    pub fn reseaon_is_none() -> Self {
        ResponseBody {
            code: 90002,
            message: format!("failed reseaon is none,please input reseaon"),
            data: None,
        }
    }
    pub fn status_type_error() -> Self {
        ResponseBody {
            code: 90003,
            message: format!("Validation status error,input \"success\" or \"failed\""),
            data: None,
        }
    }
    pub fn return_none_error() -> Self {
        ResponseBody {
            code: 90004,
            message: format!("获取请求头为空"),
            data: None,
        }
    }
    ///处理代码中信息解析unwrap错误信息
    pub fn object_not_exit() -> Self {
        ResponseBody {
            code: 90005,
            message: format!("查询对象不存在"),
            data: None,
        }
    }
}