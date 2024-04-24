use crate::app;

#[cfg(test)]
mod tests {
    use crate::modelo::ControladorModelo;

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode}
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn inicio() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
