use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub enum ResponseError {
    DataNotFound,
    InvalidRequest,
    DatabaseError,
    InternalError,
    Unauthorized,
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let status = match self {
            ResponseError::DataNotFound => StatusCode::NOT_FOUND,
            ResponseError::InvalidRequest => StatusCode::BAD_REQUEST,
            ResponseError::DatabaseError => StatusCode::BAD_GATEWAY,
            ResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        let body = match self {
            ResponseError::DataNotFound => "data not found",
            ResponseError::InvalidRequest => "invalid request",
            ResponseError::DatabaseError => "database error",
            ResponseError::InternalError => "internal server error",
            ResponseError::Unauthorized => "Unauthorized"
        };

        (status, body).into_response()
    }
}
