use async_graphql::{Context, InputObject, Object, Result as GQLResult};

use crate::extractors::Authentication;
use crate::graphql::{
    subscriptions::color::{ColorMutationMode, ColorPayload},
    subscriptor::Subscriptor,
    types::color::Color,
};
use crate::prisma::color;

#[derive(InputObject)]
pub struct ColorUpdateInput {
    pub color: color::color::Type,
    pub color_code: color::color_code::Type,
}

#[derive(Default)]
pub struct ColorMutation;

#[Object]
impl ColorMutation {
    async fn edit_color(
        &self,
        ctx: &Context<'_>,
        id: i32,
        data: ColorUpdateInput,
    ) -> GQLResult<Color> {
        let auth = ctx.data::<Authentication>()?;
        let prisma = &auth.prisma_client;

        let color_data: Color = (&prisma
            .color()
            .update(
                color::id::equals(id),
                vec![
                    color::color::set(data.color.clone()),
                    color::color_code::set(data.color_code.clone()),
                ],
            )
            .exec()
            .await?)
            .into();

        Subscriptor::<ColorPayload>::publish(ColorPayload {
            mutation: ColorMutationMode::UPDATED,
            color: color_data.color.clone(),
            color_code: color_data.color_code.clone(),
        });

        Ok(color_data)
    }

    #[allow(unused)]
    async fn add_color(&self, ctx: &Context<'_>) -> GQLResult<Color> {
        todo!()
    }

    #[allow(unused)]
    async fn delete_color(&self, ctx: &Context<'_>) -> GQLResult<Color> {
        todo!()
    }
}
