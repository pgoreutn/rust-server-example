use super::UserResponseBody;
use crate::{encode_token, ResponseError, State};
use repository::domain::UserRepository;
use serde::Deserialize;
use tide::{http_types::StatusCode, Request, Response};

pub async fn sign_in(mut req: Request<State>) -> Result<Response, ResponseError> {
    let req_body: SignInRequestBody = req
        .body_json()
        .await
        .map_err(|e| Response::new(StatusCode::BadRequest).body_string(e.to_string()))?;

    let repository = &req.state().repository;

    let user = repository
        .get_user_by_email_and_password(&req_body.user.email, &req_body.user.password)
        .await?;

    let token = encode_token(user.id, &req.state().jwt_hs_secret)?;

    let resp_body: UserResponseBody = (user, token).into();

    let resp = Response::new(StatusCode::Ok).body_json(&resp_body)?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
struct SignInRequestBody {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    email: String,
    password: String,
}
