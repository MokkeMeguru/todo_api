use axum::{http::StatusCode, Json};

/// YAMLファイルからOpenAPI仕様を取得
#[utoipa::path(
    get,
    path = "/api-docs/yaml",
    tag = "openapi",
    responses(
        (status = 200, description = "OpenAPI specification in YAML format"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_openapi_yaml() -> Result<String, StatusCode> {
    let content = std::fs::read_to_string("spec/api/openapi.yaml")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(content)
}

/// YAMLファイルからJSON形式のOpenAPI仕様を取得
#[utoipa::path(
    get,
    path = "/api-docs/yaml/json",
    tag = "openapi",
    responses(
        (status = 200, description = "OpenAPI specification in JSON format"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_openapi_yaml_json() -> Result<Json<serde_json::Value>, StatusCode> {
    let yaml_content = std::fs::read_to_string("spec/api/openapi.yaml")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let yaml_value: serde_json::Value = serde_yaml::from_str(&yaml_content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(yaml_value))
} 