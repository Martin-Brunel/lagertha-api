use rocket::{http::Status, response::status::Custom, serde::json::Json, Request};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const DEFAULT_ERROR_MESSAGE: &str = "__DEFAULT__";

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ErrorObject {
    pub code: u16,
    pub message: String,
}

impl ErrorObject {
    pub fn new(message: String, status_code: u16) -> Self {
        ErrorObject {
            message,
            code: status_code,
        }
    }

    pub fn create(status: Status, msg: Option<&str>) -> CustomError {
        let reason = match msg {
            None => match status.reason() {
                None => "",
                Some(reason) => reason
            },
            Some(msg) =>msg
        };
        Custom(status, Json(ErrorObject::new(reason.to_string(), status.code)))
    }
}

pub type CustomError = Custom<Json<ErrorObject>>;

// #[catch(401)]
// pub async fn unauthorized(req: &Request<'_>) -> Json<ErrorObject> {
//     let (_, todo_error) = req.guard::<Security>().await.failed().unwrap();

//     Json(todo_error)
// }

// #[catch(403)]
// pub async fn forbidden(req: &Request<'_>) -> Json<ErrorObject> {
//     match req.guard::<Security>().await.failed() {
//         Some((_, todo_error)) => Json(todo_error),
//         None => Json(ErrorObject {
//             message: String::from("Not authorized"),
//         }),
//     }
// }

#[catch(404)]
pub async fn notfound() -> Json<ErrorObject> {
    Json(ErrorObject {
        message: String::from("Not found"),
        code: 404,
    })
}

#[catch(422)]
pub fn unprocessable_entity(_status: Status, req: &Request) -> Json<ErrorObject> {
    let possible_reason = req
        .local_cache(|| ErrorObject {
            message: DEFAULT_ERROR_MESSAGE.into(),
            code: 422,
        })
        .to_owned();
    Json(possible_reason)
}
