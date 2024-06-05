use crate::helper::error::ResponseError;
use crate::interface;
use crate::repository::post::{
    delete_post_by_id, get_all_post, get_post_by_id, publish_post_by_id,
};
use axum::{
    extract::{Json as Req, Path},
    response::Json,
};
use template_example::*;
use models::*;
use interface::post::{CreatePost, DeletePost, PostId, ResponseDeletePost, ResponsePost};
use validator::Validate;

pub async fn show_post() -> Json<Vec<Post>> {
    let results = get_all_post();

    Json(results)
}

pub async fn get_post(Path(post_id): Path<i32>) -> Json<ResponsePost> {
    let post = get_post_by_id(post_id);

    let res = match post {
        Ok(Some(post)) => ResponsePost {
            status: 200,
            data: Some(post),
        },
        Ok(None) => ResponsePost {
            status: 404,
            data: None,
        },
        Err(_) => ResponsePost {
            status: 500,
            data: None,
        },
    };

    Json(res)
}

pub async fn write_post(
    Req(payload): Req<CreatePost>,
) -> Result<Json<ResponsePost>, ResponseError> {
    match payload.validate() {
        Ok(_) => (),
        Err(_) => return Err(ResponseError::InvalidRequest),
    };
    let title = payload.title;
    let body = payload.body;
    let post = create_post(title.as_str(), body.as_str());
    match post {
        Ok(Some(post)) => {
            let res = ResponsePost {
                status: 200,
                data: Some(post),
            };
            Ok(Json(res))
        }
        Ok(None) => Err(ResponseError::DataNotFound),
        Err(_) => Err(ResponseError::DatabaseError),
    }
}

pub async fn published_post(
    Req(payload): Req<PostId>,
) -> Result<Json<ResponsePost>, ResponseError> {
    let id = payload.id;
    let post = publish_post_by_id(id);
    match post {
        Ok(post) => {
            let res = ResponsePost {
                status: 200,
                data: Some(post),
            };
            Ok(Json(res))
        }
        Err(_) => Err(ResponseError::DataNotFound),
    }
}

pub async fn delete_post(
    Req(payload): Req<DeletePost>,
) -> Result<Json<ResponseDeletePost>, ResponseError> {
    let target = payload.title;
    let count = delete_post_by_id(target);

    match count > 0 {
        true => {
            let res = ResponseDeletePost {
                status: 200,
                message: format!("Delete {} posts", count),
            };
            Ok(Json(res))
        }
        false => Err(ResponseError::DataNotFound),
    }
}

