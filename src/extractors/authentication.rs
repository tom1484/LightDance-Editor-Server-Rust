use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use redis::Client;
use std::sync::Arc;

use crate::graphql::schema::AppState;
use crate::prisma::{
    editing_control_frame, editing_led_effect, editing_position_frame, user, PrismaClient,
};

#[derive(Debug)]
pub struct Authentication {
    pub username: String,
    pub user_id: i32,
    pub prisma_client: Arc<PrismaClient>,
    pub redis_client: Arc<Client>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let AppState { prisma, redis } = crate::APP_STATE.get().unwrap();

        let test_user = prisma.user().find_first(vec![]).exec().await.unwrap();
        if let Some(test_user) = test_user {
            let editing_control = prisma
                .editing_control_frame()
                .find_first(vec![editing_control_frame::user_id::equals(test_user.id)])
                .exec()
                .await
                .unwrap();
            if editing_control.is_none() {
                prisma.editing_control_frame().create(
                    user::UniqueWhereParam::IdEquals(test_user.id),
                    vec![
                        editing_control_frame::frame_id::set(None),
                        editing_control_frame::user_id::set(test_user.id),
                    ],
                );
            }

            let editing_position = prisma
                .editing_control_frame()
                .find_first(vec![editing_control_frame::user_id::equals(test_user.id)])
                .exec()
                .await
                .unwrap();
            if editing_position.is_none() {
                prisma.editing_position_frame().create(
                    user::UniqueWhereParam::IdEquals(test_user.id),
                    vec![
                        editing_position_frame::frame_id::set(None),
                        editing_position_frame::user_id::set(test_user.id),
                    ],
                );
            }

            let editing_led = prisma
                .editing_led_effect()
                .find_first(vec![editing_led_effect::user_id::equals(test_user.id)])
                .exec()
                .await
                .unwrap();
            if editing_led.is_none() {
                prisma.editing_led_effect().create(
                    user::UniqueWhereParam::IdEquals(test_user.id),
                    vec![
                        editing_led_effect::led_effect_id::set(None),
                        editing_led_effect::user_id::set(test_user.id),
                    ],
                );
            }

            Ok(Authentication {
                username: String::from(test_user.name),
                user_id: test_user.id,
                prisma_client: prisma.clone(),
                redis_client: redis.clone(),
            })
        } else {
            Err("No test user found")
        }
    }
}

impl Drop for Authentication {
    fn drop(&mut self) {
        let prisma = &self.prisma_client;

        prisma.editing_control_frame().update(
            editing_control_frame::user_id::equals(self.user_id),
            vec![editing_control_frame::frame_id::set(None)],
        );

        prisma.editing_position_frame().update(
            editing_position_frame::user_id::equals(self.user_id),
            vec![editing_position_frame::frame_id::set(None)],
        );

        prisma.editing_led_effect().update(
            editing_led_effect::user_id::equals(self.user_id),
            vec![editing_led_effect::led_effect_id::set(None)],
        );
    }
}
