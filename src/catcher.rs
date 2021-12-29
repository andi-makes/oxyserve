use actix_web::{middleware::errhandlers::{ErrorHandlers, ErrorHandlerResponse}, dev::ServiceResponse, http::StatusCode};
use actix_web::{web, Result};
use actix_http::{body::Body, Response};
use handlebars::Handlebars;

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
            let config = match crate::config::Config::from_file(&format!("{}/404.json", data_dir)) {
                Ok(c) => c,
                Err(_) => return fallback(error),
            };
            let body = hb.render("404", &config.context);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
