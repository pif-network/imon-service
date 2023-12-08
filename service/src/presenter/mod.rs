use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use bb8_redis::redis;
use serde::{Deserialize, Serialize};

use libs::record::{Task, TaskState};

pub mod handlers;
pub mod logic;

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreTaskPayload {
    user_name: String,
    task: Task,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRecordPayload {
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetUserDataPayload {
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskLogPayload {
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskPayload {
    key: String,
    state: TaskState,
}

fn construct_redis_error_response(err: redis::RedisError) -> serde_json::Value {
    match err.kind() {
        redis::ErrorKind::ResponseError => serde_json::json!({
            "status": "error",
            // FIXME: Most of the time, this error means that the user has not
            // registered yet, but it is still not the best way to handle.
            "message": "Invalid credentials",
        }),
        _ => serde_json::json!({
            "status": "error",
            "message": err.to_string(),
        }),
    }
}

fn construct_json_error_response(
    err: &JsonRejection,
) -> (StatusCode, axum::Json<serde_json::Value>) {
    match err {
        case @ JsonRejection::JsonDataError(_) => {
            let p = serde_json::json!({
                "status": "error",
                "message": "Invalid JSON",
                "error": format!("{:?}", case.body_text()),
            });
            (StatusCode::BAD_REQUEST, Json(p))
        }
        _ => {
            let p = serde_json::json!({
                "status": "error",
                "message": "Unknown error",
            });
            (StatusCode::BAD_REQUEST, Json(p))
        }
    }
}
