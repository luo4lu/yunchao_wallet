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
}