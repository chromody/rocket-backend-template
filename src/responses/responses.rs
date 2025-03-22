use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody {
    data: Option<serde_json::Value>,
    message: Option<String>,
}

//wrapping our response body and turning it into a json object
#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}

pub fn create_response(data: Option<serde_json::Value>, message: Option<String>) -> Response {
    Response {
        body: ResponseBody {
            data: data,
            message: message,
        }
    }
}

//macro for creating a new response
#[macro_export]
macro_rules! new_response {
    ($data:expr) => {
        crate::responses::responses::create_response(Some($data), None)
    };
    ($data:expr, $message:expr) => {
        crate::responses::responses::create_response(Some($data), Some($message))
    };
    () => {
        crate::responses::responses::create_response(None, None)
    };
    ($message:expr) => {
        crate::responses::responses::create_response(None, Some($message))
    };
}