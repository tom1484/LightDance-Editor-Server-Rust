use async_graphql::Schema;
use std::sync::Arc;
use dotenv::var;
use redis::Client;

use crate::{
    graphql::{MutationRoot, QueryRoot, SubscriptionRoot},
    prisma::PrismaClient,
};

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub async fn build_schema() -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .finish()
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: Arc<PrismaClient>,
    pub redis: Arc<Client>,
}

pub async fn build_prisma_client() -> Arc<PrismaClient> {
    let prisma_client = PrismaClient::_builder()
        .build()
        .await
        .expect("Failed to create Prisma client");

    #[cfg(debug_assertions)]
    prisma_client
        ._db_push()
        .accept_data_loss()
        .await
        .expect("Failed to push database schema");

    // #[cfg(not(debug_assertions))]
    // client
    //     ._migrate_deploy()
    //     .await
    //     .expect("Failed to deploy database schema");

    Arc::new(prisma_client)
}

pub async fn build_redis_client() -> Arc<Client> {
    let redis_host = var("REDIS_HOST").expect("REDIS_HOST is not set");
    let redis_port = var("REDIS_PORT").expect("REDIS_PORT is not set");

    let redis_client = Client::open(format!("redis://{}:{}", redis_host, redis_port))
        .expect("Failed to create redis client");

    Arc::new(redis_client)
}
