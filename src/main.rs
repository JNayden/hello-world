use axum::{
    routing::get, 
    Router, 
    response::{Html, IntoResponse},
    Json,
    response::Redirect
};
use serde::Serialize;
use std::net::SocketAddr;
use tera::{Tera, Context};
use axum::http::StatusCode;
use std::convert::Infallible;
// Define your template structure
struct IndexTemplate;

#[derive(Serialize)]
struct UserAnalytics {
    user_id: u32,
    posts_count: u32,
    likes_count: u32,
    reputation: f32,
}
async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn store() -> &'static str {
    "Hello, Store!"
}
async fn redirect_to_olx() -> Redirect {
    Redirect::temporary("https://olx.bg") // 302 redirect to /hello-world
}
async fn kebapa_papa() -> Redirect {
    Redirect::temporary("https://glovoapp.com/bg/bg/varna/kebapa-papa-dunners-and-sandwiches-2-var/") // 302 redirect to /hello-world
}

// Handler for analytics
async fn get_user_analytics() -> Json<UserAnalytics> {
    let analytics = UserAnalytics {
        user_id: 1,
        posts_count: 120,
        likes_count: 340,
        reputation: 4.8,
    };

    Json(analytics)
}

// Serve the HTML page dynamically using Tera templates
async fn serve_frontend() -> Result<Html<String>, Infallible> {
    // Load the Tera templates
    let tera = Tera::new("templates/**/*").unwrap();

    // Create a context and pass the analytics data
    let mut context = Context::new();
    context.insert("user_id", &1);
    context.insert("posts_count", &120);
    context.insert("likes_count", &340);
    context.insert("reputation", &4.8);

    // Render the template with data
    let rendered = tera.render("index.html", &context).unwrap();
    
    // Return the rendered HTML
    Ok(Html(rendered))
}

#[tokio::main]
async fn main() {
    // Define the routes and handlers
    let app = Router::new()
        .route("/", get(serve_frontend)) // Serve the dynamic frontend
        .route("/api/user_analytics", get(get_user_analytics)) // API endpoint for user analytics
    .route("/store", get(store))
    .route("/qr/fidgetcubes", get(redirect_to_olx))
    .route("/qr/kebapa", get(kebapa_papa));

    // Set the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
