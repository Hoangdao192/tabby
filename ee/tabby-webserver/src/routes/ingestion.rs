use std::sync::Arc;

use axum::{
    extract::{Path, State, Multipart},
    Json,
};
use docx_rust::document::Text;
use futures::TryStreamExt;
use hyper::StatusCode;
use tabby_common::api::ingestion::{IngestionRequest, IngestionResponse};
use tabby_schema::ingestion::IngestionService;
use tracing::{error, instrument, warn};
use validator::Validate;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::service::extractor::text_extractor::TextExtractor;

pub struct IngestionState {
    pub ingestion: Arc<dyn IngestionService>,
}

#[utoipa::path(
    post,
    path = "/v1beta/ingestion",
    operation_id = "ingestion",
    tag = "v1beta",
    responses(
        (status = 202, description = "Accepted, Waiting to be processed", content_type = "application/json"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("token" = [])
    )
)]
#[instrument(skip(state, request))]
pub async fn ingestion(
    State(state): State<Arc<IngestionState>>,
    Json(request): Json<IngestionRequest>,
) -> Result<(StatusCode, Json<IngestionResponse>), StatusCode> {
    if let Err(e) = request.validate() {
        warn!("Invalid request: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    let response = state
        .ingestion
        .ingestion(request.clone())
        .await
        .map_err(|e| {
            error!("Failed to process ingestion: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::ACCEPTED, Json(response)))
}

#[utoipa::path(
    post,
    path = "/v1beta/ingestion/index",
    operation_id = "ingestion",
    tag = "v1beta",
    responses(
        (status = 202, description = "Accepted, Waiting to be processed", content_type = "application/json"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("token" = [])
    )
)]
#[instrument(skip(state, request))]
pub async fn ingestion_and_index(
    State(state): State<Arc<IngestionState>>,
    mut multipart: Multipart
) -> Result<(StatusCode, Json<IngestionResponse>), StatusCode> {
    let file_name: String;
    let mut file: File;
    let mut file_path: &std::path::Path;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name == "file" {
            file_name = field.file_name().unwrap_or("").to_string();
            let content_type = field.content_type().unwrap_or("").to_string();
            file_path = std::path::Path::new(&format!("/tmp/{}", file_name));
            file = match File::create(&file_path).await {
                Ok(file) => file,
                Err(_) => {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let mut mutable_field = field;
            while let Some(chunk) = mutable_field.chunk().await.unwrap() {
                if let Err(e) = file.write_all(&chunk).await {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }

    let text_extractor: dyn TextExtractor = TextExtractor::detect_extractor(file_path);
    
    let mut request: IngestionRequest = IngestionRequest {
        source: uuid::Uuid::new_v4().to_string(),
        id: uuid::Uuid::new_v4().to_string(),
        title: file_name,
        body: "".to_string(),
        link: None,
        ttl:None
    };

    if let Err(e) = request.validate() {
        warn!("Invalid request: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    let response = state
        .ingestion
        .ingestion_multipart(

        )
        .await
        .map_err(|e| {
            error!("Failed to process ingestion: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((StatusCode::ACCEPTED, Json(response)))
}

#[utoipa::path(
    delete,
    path = "/v1beta/ingestion/{source}",
    operation_id = "delete_ingestion_source",
    tag = "v1beta",
    responses(
        (status = 202, description = "Accepted, Waiting to be processed", content_type = "application/json"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("token" = [])
    )
)]
#[instrument(skip(state))]
pub async fn delete_ingestion_source(
    State(state): State<Arc<IngestionState>>,
    Path(source): Path<String>,
) -> Result<StatusCode, StatusCode> {
    if source.is_empty() {
        warn!("Invalid request: source is empty");
        return Err(StatusCode::BAD_REQUEST);
    }

    state
        .ingestion
        .delete_by_source_id(source)
        .await
        .map_err(|e| {
            error!("Failed to process ingestion: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::ACCEPTED)
}

#[utoipa::path(
    delete,
    path = "/v1beta/ingestion/{source}/{id}",
    operation_id = "delete_ingestion",
    tag = "v1beta",
    responses(
        (status = 202, description = "Accepted, Waiting to be processed", content_type = "application/json"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("token" = [])
    )
)]
#[instrument(skip(state))]
pub async fn delete_ingestion(
    State(state): State<Arc<IngestionState>>,
    Path((source, id)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
    if source.is_empty() || id.is_empty() {
        warn!("Invalid request: source or id is empty");
        return Err(StatusCode::BAD_REQUEST);
    }

    state.ingestion.delete(source, id).await.map_err(|e| {
        error!("Failed to process ingestion: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::ACCEPTED)
}
