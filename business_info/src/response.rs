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
            message: format!("数据库操作失败or请求unwrap:{}", error),
            data: None,
        }
    }
    ///该企业已经注册过
    pub fn enterprise_registed() -> Self {
        ResponseBody {
            code: 90001,
            message: format!("企业信息已经注册过"),
            data: None,
        }
    }
    ///企业信息认证失败
    pub fn enterprise_info_error() -> Self {
        ResponseBody {
            code: 90002,
            message: format!("企业信息认证失败"),
            data: None,
        }
    }
    pub fn personal_info_error() -> Self {
        ResponseBody {
            code: 90003,
            message: format!("法人信息认证失败"),
            data: None,
        }
    }
    pub fn agency_info_error() -> Self {
        ResponseBody {
            code: 90004,
            message: format!("代理人信息认证失败"),
            data: None,
        }
    }
    pub fn user_data_error() -> Self {
        ResponseBody {
            code: 90005,
            message: format!("邮箱服务处理用户数据失败"),
            data: None,
        }
    }
    pub fn agency_info_miss() -> Self {
        ResponseBody {
            code: 90006,
            message: format!("代理人身份信息不完整"),
            data: None,
        }
    }
}