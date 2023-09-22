use async_graphql::{Context, Object, Result as GQLResult};

use crate::extractors::Authentication;
use crate::graphql::types::{ColorMap, ColorMapScalar};

#[derive(Default)]
pub struct ColorQuery;

#[Object]
impl ColorQuery {
    async fn color_map(&self, ctx: &Context<'_>) -> GQLResult<ColorMap> {
        #[allow(unused)]
        let auth = ctx.data::<Authentication>()?;
        let prisma = &auth.prisma_client;

        let result = prisma
            .color()
            .find_many(vec![])
            .exec()
            .await?
            .iter()
            .map(|data| (data.id, data.into()))
            .collect();

        Ok(ColorMap {
            color_map: ColorMapScalar(result),
        })
    }
}
