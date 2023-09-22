pub mod extractors;
pub mod graphql;
pub mod prisma;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
#[cfg(debug_assertions)]
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use std::sync::Arc;

use extractors::Authentication;
use graphql::schema::{build_prisma_client, build_redis_client, build_schema, AppSchema, AppState};

async fn graphql(
    auth: Authentication,
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth)).await.into()
}

async fn graphql_explorer() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/api/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = build_schema().await;
    let prisma_client = build_prisma_client().await;
    let redis_client = build_redis_client().await;

    APP_STATE
        .set(AppState {
            prisma: Arc::clone(&prisma_client),
            redis: Arc::clone(&redis_client),
        })
        .unwrap();

    let app = Router::new()
        .route("/api/graphql", get(graphql_explorer).post(graphql))
        .route("/playground", get(graphql_playground))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));

    println!("Playground: http://localhost:5001/api/graphql");

    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
