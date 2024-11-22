use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use maud::{html, Markup};
use std::{collections::HashMap, fs, net::SocketAddr};
use strsim::levenshtein;

static VALID_PATHS: &[&str] = &[
    "/", "/about", "/contact", "/blog", "/blog/posts", "/announcements"
];

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/about", get(about))
        .route("/contact", get(contact))
        .route("/blog", get(blog))
        .route("/blog/posts", get(blog_posts))
        .route("/dyn", get(blog_page))
        .fallback(get(not_found_handler)); // Catch-all for unknown paths

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn homepage() -> impl IntoResponse {
    let html = fs::read_to_string("static/html/index.html").unwrap();
    Html(html)
}

async fn about() -> impl IntoResponse{
    Html("<h1>About Us</h1>")
}

async fn contact() ->impl IntoResponse {
    Html("<h1>Contact Us</h1>")
}

async fn blog() -> impl IntoResponse {
    let html = fs::read_to_string("static/html/blog.html").unwrap();
    Html(html)
}

async fn blog_posts() -> impl IntoResponse {
    Html("<h1>Blog Posts</h1>")
}
async fn announcements() -> impl IntoResponse{
    let html = fs::read_to_string("static/html/announcements.html").unwrap();
    Html(html)
}

async fn not_found_handler(uri: axum::http::Uri) -> impl IntoResponse {
    let mistyped_path = uri.path();
    if let Some(suggestion) = suggest_path(mistyped_path) {
        Html(format!("<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/css/bootstrap.min.css\" rel=\"stylesheet\">
            <title>Simple Blog</title>
        </head>
        <body>
            <div class=\"container mt-5\">
                <header class=\"mb-4\">
                    <h1 class=\"text-center\"><h1>404 Not Found</h1><p>Did you mean <a href=\"{}\">{}</a>?</p>
                </header>
                <article class=\"card mb-3\">
                    <div class=\"card-body\">
                        <h2 class=\"card-title\">Welcome to the Blog!</h2>
                        <p class=\"card-text\">This is a simple blog page powered by Rust and Axum with Bootstrap styling. You can expand this to include dynamic content, user accounts, and more!</p>
                    </div>
                </article>
                <footer class=\"text-center\">
                    <p>&copy; 2024 Simple Blog. Powered by Rust and Bootstrap.</p>
                </footer>
            </div>
        </body>
        </html>
        ",
        suggestion, suggestion
    ))
    } else {
        Html(format!("<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/css/bootstrap.min.css\" rel=\"stylesheet\">
            <title>Simple Blog</title>
        </head>
        <body>
            <div class=\"container mt-5\">
                <header class=\"mb-4\">
                    <h1 class=\"text-center\"><h1>404 Not Found</h1><p>Sorry, we couldn't find the page you were looking for.</p>
                </header>
                <article class=\"card mb-3\">
                    <div class=\"card-body\">
                        <h2 class=\"card-title\">Welcome to the Blog!</h2>
                        <p class=\"card-text\">This is a simple blog page powered by Rust and Axum with Bootstrap styling. You can expand this to include dynamic content, user accounts, and more!</p>
                    </div>
                </article>
                <footer class=\"text-center\">
                    <p>&copy; 2024 Simple Blog. Powered by Rust and Bootstrap.</p>
                </footer>
            </div>
        </body>
        </html>
        "
    ))
    }
}

async fn blog_page() -> impl IntoResponse {
    Html(render_blog_page().into_string())
}

// Use `maud` to render the blog page dynamically
fn render_blog_page() -> Markup {
    html! {
        (page_header("My Blog Page"))
        div class="blog-container" {
            h1 { "Welcome to the Blog!" }
            p {
                "This is a brief introduction to the blog content. "
                a href="javascript:void(0);" onclick="toggleContent();" {
                    "Read morea"
                }
            }
            div id="hidden-content" style="display: none; overflow: hidden; transition: max-height 0.5s ease;" {
                p {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                }
            }
        }
        (inline_script())
    }
}

// Render a basic page header
fn page_header(title: &str) -> Markup {
    html! {
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { (title) }
            style {
                "body { font-family: Arial, sans-serif; margin: 20px; }"
                ".blog-container { max-width: 600px; margin: auto; }"
                "#hidden-content { max-height: 0; }"
                ".show { max-height: 300px; }"
            }
        }
    }
}

// Inline JavaScript for toggling content visibility
fn inline_script() -> Markup {
    html! {
        script {
            r#"
            document.addEventListener("DOMContentLoaded", function() {
                function toggleContent() {
                    let content = document.getElementById('hidden-content');
                    if (content.style.display === 'none' || content.style.display === '') {
                        content.style.display = 'block';
                        content.classList.add('show');
                    } else {
                        content.style.display = 'none';
                        content.classList.remove('show');
                    }
                }
                window.toggleContent = toggleContent; // Make the function globally accessible
            });
            "#
        }
    }
}


fn suggest_path(mistyped_path: &str) -> Option<&'static str> {
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for &valid_path in VALID_PATHS {
        let distance = levenshtein(mistyped_path, valid_path);
        if distance < best_distance {
            best_distance = distance;
            best_match = Some(valid_path);
        }
    }

    if best_distance <= 3 {
        best_match
    } else {
        None
    }
}
