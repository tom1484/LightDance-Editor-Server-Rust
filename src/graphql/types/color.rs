use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::prisma::color;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Color {
    pub id: color::id::Type,
    pub color: color::color::Type,
    pub color_code: color::color_code::Type,
}

impl From<&color::Data> for Color {
    fn from(data: &color::Data) -> Self {
        Self {
            id: data.id.clone(),
            color: data.color.clone(),
            color_code: data.color_code.clone(),
        }
    }
}
