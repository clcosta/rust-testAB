use crate::routes::contracts::{BaseResponse, Data, SlugPayLoad, SlugVariantPayLoad};
use axum::{extract::Path, response::{IntoResponse, Redirect}, Json};

pub async fn basic_view() -> impl IntoResponse {
    "Get out of here"
}

pub async fn create_slug_view(Json(data): Json<SlugPayLoad>) -> impl IntoResponse {
    let response = BaseResponse::new(
        200,
        "".to_string(),
        Some(Data::new(std::collections::HashMap::from([(
            "slug".to_string(),
            data.slug.to_string(),
        )]))),
    );
    Json(response)
}

pub async fn create_slug_variant_view(Json(data): Json<SlugVariantPayLoad>) -> impl IntoResponse {
    let mut response_data = Data::new(std::collections::HashMap::from([(
        "slug".to_string(),
        data.slug.to_string(),
    )]));
    response_data
        .data
        .insert("variant_name".to_string(), data.variant_name);
    response_data.data.insert("url".to_string(), data.url);
    let response = BaseResponse::new(200, "".to_string(), Some(response_data));
    Json(response)
}

pub async fn redirect_to_view(Path(slug): Path<String>) -> impl IntoResponse {
    println!("{}", slug);
    Redirect::permanent("/")
}
