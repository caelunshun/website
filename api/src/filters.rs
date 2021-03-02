use crate::rejections;
use semver::Version;
use sqlx::{query, PgPool};
use warp::{Filter, Rejection};

use crate::handlers;

fn with_authorized(pool: PgPool) -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
    warp::header::header("Authorization").and_then(move |token: String| {
        let pool = pool.clone();
        async move {
            let mut secret = [0u8; 48];
            hex::decode_to_slice(&token, &mut secret).map_err(|_| rejections::unauthorized())?;
            let auth_token = query!(
                r#"
                    SELECT user_id FROM user_tokens WHERE secret = $1
                "#,
                &secret[..]
            )
            .fetch_one(&pool)
            .await
            .map_err(|_| rejections::unauthorized())?;

            Ok::<_, warp::reject::Rejection>(auth_token.user_id as u32)
        }
    })
}

fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn routes(pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    health(pool.clone())
        .or(get_me(pool.clone()))
        .or(login_github(pool.clone()))
}

pub fn health(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and(with_pool(pool.clone()))
        .and_then(handlers::health)
}

pub fn get_me(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("me"))
        .and(warp::path::end())
        .and(with_authorized(pool.clone()))
        .and(with_pool(pool))
        .and_then(handlers::get_me)
}

pub fn login_github(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let client_id =
        std::env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_OAUTH_CLIENT_ID missing");
    let client_secret =
        std::env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("GITHUB_OAUTH_CLIENT_SECRET missing");
    warp::get()
        .and(warp::path("me"))
        .and(warp::path("access_token"))
        .and(warp::query())
        .and(warp::path::end())
        .and(warp::any().map(move || client_id.clone()))
        .and(warp::any().map(move || client_secret.clone()))
        .and(with_pool(pool))
        .and_then(handlers::login_github)
}

pub fn delete_user(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path("me"))
        .and(with_pool(pool))
        .and_then(handlers::delete_user)
}

pub fn create_token(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("me"))
        .and(warp::path("tokens"))
        .and(warp::path::param())
        .and(with_pool(pool))
        .and_then(handlers::create_token)
}

pub fn delete_token(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path("me"))
        .and(warp::path("tokens"))
        .and(warp::path::param())
        .and(with_pool(pool))
        .and_then(handlers::delete_token)
}

pub fn update_email(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("me"))
        .and(warp::path("email"))
        .and(with_pool(pool))
        .and_then(handlers::update_email)
}

pub fn send_email_verification(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("me"))
        .and(warp::path("email"))
        .and(warp::path("verify"))
        .and(with_pool(pool))
        .and_then(handlers::send_email_verification)
}

pub fn verify_email_code(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("me"))
        .and(warp::path("email"))
        .and(warp::path("verify"))
        .and(warp::path::param())
        .and(with_pool(pool))
        .and_then(handlers::verify_email_code)
}

pub fn list_users(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("users"))
        .and(warp::query())
        .and(with_pool(pool))
        .and_then(handlers::list_users)
}

pub fn get_user(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("users"))
        .and(warp::path::param())
        .and(with_pool(pool))
        .and_then(handlers::get_user)
}

pub fn plugins_list(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("plugins"))
        .and(warp::query())
        .and(with_pool(pool))
        .and_then(handlers::plugin_list)
}

pub fn get_plugin(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("plugins"))
        .and(warp::path::param())
        .and(
            warp::path::param::<Version>()
                .map(Some)
                .or_else(|_| async { Ok::<_, std::convert::Infallible>((None,)) }),
        )
        .and(with_pool(pool))
        .and_then(handlers::get_plugin)
}

pub fn plugin_invite_owner(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("plugins"))
        .and(warp::path::param())
        .and(warp::path("invite"))
        .and(warp::path::param())
        .and(with_pool(pool))
        .and_then(handlers::plugin_invite_owner)
}
