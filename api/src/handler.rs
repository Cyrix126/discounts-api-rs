use crate::{error::AppError, AppState};
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use discounts_common::{schema::discounts::dsl::discounts, Discount};
pub async fn create_discount(
    State(state): State<AppState>,
    Json(discount): Json<Discount>,
) -> Result<impl IntoResponse, AppError> {
    // check if percentage is correct
    if discount.percentage > 100 {
        return Err(AppError::BadRequest);
    }
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        diesel::insert_into(discounts)
            .values(discount)
            .execute(conn)?;
        Ok::<(), AppError>(())
    })
    .await??;

    Ok(())
}
pub async fn read_discount(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    Ok(Json(
        conn.interact(move |conn| {
            Ok::<Discount, AppError>(
                discounts
                    .find(id as i32)
                    .select(Discount::as_select())
                    .first(conn)?,
            )
        })
        .await??,
    ))
}
pub async fn update_discount(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(mut discount): Json<Discount>,
) -> Result<impl IntoResponse, AppError> {
    // check if percentage is correct
    if discount.percentage > 100 {
        return Err(AppError::BadRequest);
    }
    // ignore id from json
    discount.id = id as i32;
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        diesel::update(discounts).set(discount).execute(conn)?;
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}
pub async fn delete_discount(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        diesel::delete(discounts.filter(discounts_common::schema::discounts::id.eq(id as i32)))
            .execute(conn)?;
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}
/// Used by cart API to check if code valid
pub async fn read_discount_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    Ok(Json(
        conn.interact(move |conn| {
            Ok::<Discount, AppError>(
                discounts
                    .filter(discounts_common::schema::discounts::code.eq(&code))
                    .select(Discount::as_select())
                    .first(conn)?,
            )
        })
        .await??,
    ))
}
/// used for public, needs anti brutforce measure, returns only if code valid in time period
pub async fn percentage_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        let discount: Discount = discounts
            .filter(discounts_common::schema::discounts::code.eq(&code))
            .select(Discount::as_select())
            .first(conn)?;
        if discount.is_time_valid() {
            Ok(Json(discount.percentage as u8))
        } else {
            Err(AppError::CodeInvalid)
        }
    })
    .await?
}
pub async fn all_discounts(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    Ok(Json(
        conn.interact(move |conn| {
            Ok::<Vec<Discount>, AppError>(discounts.select(Discount::as_select()).load(conn)?)
        })
        .await??,
    ))
}
