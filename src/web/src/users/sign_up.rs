use crate::{ResponseError, State};
use repository::{domain, domain::UserRepository};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use tide::{Request, Response};

pub async fn sign_up(mut req: Request<State>) -> Result<Response, ResponseError> {
    let req_body: SignUpRequestBody = req
        .body_json()
        .await
        .map_err(|e| Response::new(400).body_string(e.to_string()))?;

    let user: domain::UserForCreate = req_body.try_into()?;

    let repository = &req.state().repository;

    let user_id = repository.create_user(user).await?;

    // TODO
    Ok(Response::new(200).body_string(format!("{}", user_id)))
}

#[derive(Deserialize, Debug)]
struct SignUpRequestBody {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<SignUpRequestBody> for domain::UserForCreate {
    type Error = domain::UserPasswordError;

    fn try_from(r: SignUpRequestBody) -> Result<Self, Self::Error> {
        let user = Self {
            username: r.user.username,
            password: domain::UserPassword::from_clear_text(r.user.password)?,
            email: r.user.email,
        };
        Ok(user)
    }
}

impl From<domain::CreateUserError> for ResponseError {
    fn from(_: domain::CreateUserError) -> Self {
        // TODO, set body
        let resp = Response::new(500);
        Self::new(resp)
    }
}
