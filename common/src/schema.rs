// @generated automatically by Diesel CLI.

diesel::table! {
    discounts (id) {
        id -> Int4,
        code -> Text,
        percentage -> Int2,
        date_begin -> Nullable<Timestamptz>,
        date_end -> Nullable<Timestamptz>,
    }
}
