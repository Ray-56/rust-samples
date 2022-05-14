use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use axum::{
    http::StatusCode,
    routing::{get, post},
    extract::{FromRequest, RequestParts, TypedHeader},
    response::{IntoResponse, Html},
    headers::{Authorization, authorization::Bearer},
    async_trait,
    Router, Server, Json,
};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;

const SECRET: &[u8] = b"deadbeef";

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: usize,
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(create_todo_handler))
        .route("/login", post(login_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello, world!")
}

async fn todos_handler() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            id: 1,
            title: "Todo 1".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Todo 2".to_string(),
            completed: true,
        },
    ])
}

async fn create_todo_handler(claims: Claims, Json(_todo): Json<CreateTodo>) -> StatusCode {
    println!("{:?}", claims);
    StatusCode::CREATED
}

async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login info validation
    let claims = Claims {
        id: 1,
        name: "Ray Fan".to_string(),
    };

    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send, // required by `async_trait`
{
    type Rejection = HttpError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = 
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| HttpError::Auth)?;
        
        let key = jwt::DecodingKey::from_secret(SECRET);
        let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|_| HttpError::Auth)?;

        Ok(token.claims)
    }
}

#[derive(Debug)]
pub enum HttpError {
    Auth,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (code, msg).into_response()
    }
}