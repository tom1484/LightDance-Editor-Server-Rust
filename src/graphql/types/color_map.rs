use async_graphql::SimpleObject;
use async_graphql::{Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::graphql::types::Color;
use crate::prisma::color;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TColorMapColor {
    #[serde(rename = "color")]
    pub color: color::color::Type,
    #[serde(rename = "colorCode")]
    pub color_code: color::color_code::Type,
}

impl From<&Color> for TColorMapColor {
    fn from(data: &Color) -> Self {
        Self {
            color: data.color.clone(),
            color_code: data.color_code.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ColorMapScalar(pub HashMap<i32, Color>);

#[Scalar]
impl ScalarType for ColorMapScalar {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        Ok(async_graphql::from_value(value)?)
    }

    fn to_value(&self) -> Value {
        Value::Object(
            self.0
                .iter()
                .map(|(k, v)| {
                    (
                        async_graphql::Name::new(k.to_string()),
                        async_graphql::to_value(TColorMapColor::from(v)).unwrap(),
                    )
                })
                .collect(),
        )
    }
}

#[derive(SimpleObject)]
pub struct ColorMap {
    pub color_map: ColorMapScalar,
}
