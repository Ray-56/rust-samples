use axum::{
    async_trait,
    body::{boxed, Full},
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    AddExtensionLayer, Json, Router, Server,
};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

const SECRET: &[u8] = b"deadbeef";
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub user_id: usize,
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
    exp: usize,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    items: Arc<RwLock<Vec<Todo>>>,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();
        match Assets::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .body(boxed(Full::from(format!("File not found: {}", path))))
                .unwrap(),
        }
    }
}

#[tokio::main]
async fn main() {
    let store = TodoStore {
        items: Arc::new(RwLock::new(vec![Todo {
            id: 0,
            user_id: 0,
            title: "Learn Rust".to_string(),
            completed: false,
        }])),
    };
    let app = Router::new()
        .route("/", get(index_handler))
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todo_handler)
                .layer(AddExtensionLayer::new(store)),
        )
        .route("/login", post(login_handler))
        .fallback(get(static_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> impl IntoResponse {
    StaticFile("index.html")
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

async fn todos_handler(
    claims: Claims,
    Extension(store): Extension<TodoStore>,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claims.id;
    match store.items.read() {
        Ok(items) => Ok(Json(
            items
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .map(|todo| todo.clone())
                .collect(),
        )),
        Err(_) => Err(HttpError::Internal),
    }
}

async fn create_todo_handler(
    claims: Claims,
    Json(todo): Json<CreateTodo>,
    Extension(store): Extension<TodoStore>,
) -> Result<StatusCode, HttpError> {
    println!("{:?}", claims);
    match store.items.write() {
        Ok(mut guard) => {
            let todo = Todo {
                id: get_next_id(),
                user_id: claims.id,
                title: todo.title,
                completed: false,
            };
            guard.push(todo);
            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(HttpError::Internal),
    }
}

async fn login_handler(Json(_login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login info validation
    let claims = Claims {
        id: 1,
        name: "Ray Fan".to_string(),
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };

    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
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

fn get_epoch() -> usize {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
