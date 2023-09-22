use async_graphql::{Enum, SimpleObject, Subscription};
use futures_core::stream::Stream;

use crate::prisma::color;

use crate::graphql::subscriptor::Subscriptor;

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
pub enum ColorMutationMode {
    UPDATED,
    CREATED,
    DELETED,
}

#[derive(SimpleObject, Clone)]
pub struct ColorPayload {
    pub mutation: ColorMutationMode,
    pub color: color::color::Type,
    pub color_code: color::color_code::Type,
}

#[derive(Default)]
pub struct ColorSubscription;

#[Subscription]
impl ColorSubscription {
    async fn color_subscription(&self) -> impl Stream<Item = ColorPayload> {
        Subscriptor::<ColorPayload>::subscribe()
    }
}
