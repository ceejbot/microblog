//! Implementation of all of our microblogging service route handlers.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Result};
use axum::Json;

use crate::status::*;
use crate::AppState;

/// Fetch a single microblog status from the db. Responds with 200 and json if found,
/// 400 otherwise.
pub async fn read(
    // This is an axum "extractor" that provides our shared state to this handler.
    State(state): State<Arc<AppState>>,
    // This is how we extract named parameters from url paths.
    Path(id): Path<String>,
    // This is our response type, but it's clunky. Can we do better?
) -> Result<Response, (StatusCode, String)> {
    // for extra credit, handle if-modified-since
    // for extra extra credit, generate an etag header outgoing, and handle if-none-match
    // how do we set headers in axum responses anyway?
    let db = state.db.clone();
    let maybe_status = sqlx::query_as!(
        StatusPublic,
        r#"
        SELECT
            id, created, modified, body
        FROM statuses
        WHERE
            id = $1
            AND
            deleted IS NULL
        "#,
        id
    )
    .fetch_optional(&db)
    .await
    // a function on Result that lets us transform any errors to another type
    .map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "We need a cute failure animal.".to_string(),
        )
    })?; // and we return Err<OurMappedType> here via the ? sugar

    // Where did this `into_response()` function come from anyway?
    if let Some(status) = maybe_status {
        Ok(Json(status).into_response()) // oh no an error
    } else {
        Ok(StatusCode::NOT_FOUND.into_response())
    }
}

/// Create a new microblog status from a json body. The macro
/// is an axum development convenience that helps us debug our types.
#[axum_macros::debug_handler]
pub async fn create(
    State(state): State<Arc<AppState>>,
    body: String,
    // The return type here is inadequate. Can we improve it?
) -> Result<(StatusCode, String)> {
    // store it in the database
    // respond with 201 Created
    // the url of the new status in the Location: header
    // extra credit: don't store a zero-length status
    todo!()
}

/// Update an existing status with a new body.
#[axum_macros::debug_handler]
pub async fn update(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
    _body: String,
    // Our return type here is "we won't tell you exactly what it is, but we do promise
    // it implements the trait `IntoResponse`". This lets us vary what concrete type we
    // return, so long as we maintain that contract. HOWEVER... it's still not good enough,
    // as we'll see.
) -> impl IntoResponse {
    // find the status with that id
    // respond with 404 if not found
    // if found, update the status body & the modified ts
    // respond with 200 OK plus updated status
    // extra credit: require the modified timestamp as a poor man's vector clock
    // and don't update if the db version is newer
    todo!()
}

/// Mark a status as deleted with a tombstone.
#[axum_macros::debug_handler]
pub async fn delete(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    // find the status with that id
    // respond with 404 if not found
    // if found, mark as deleted
    // respond with 204 No Content
    todo!()
}

/// List all statuses.
#[axum_macros::debug_handler]
pub async fn list(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // fetch all undeleted statuses from the db
    // in reverse chronological order
    // respond with 200 + a json list
    // extra credit: paginate the list
    todo!()
}
