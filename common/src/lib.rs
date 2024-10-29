use cfg_if::cfg_if;
use chrono::offset::Utc;
use chrono::DateTime;

cfg_if! {
    if #[cfg(feature="server")] {
pub mod schema;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
    }
    }
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(Queryable, Selectable, Insertable, AsChangeset)
)]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::discounts))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Discount {
    #[cfg_attr(feature = "server", diesel(skip_insertion))]
    pub id: i32,
    pub code: String,
    pub percentage: i16,
    pub date_begin: Option<DateTime<Utc>>,
    pub date_end: Option<DateTime<Utc>>,
}

impl Discount {
    pub fn is_time_valid(&self) -> bool {
        if let Some(begin) = self.date_begin {
            let now = Utc::now();
            if begin > now {
                return false;
            }
        }
        if let Some(end) = self.date_end {
            let now = Utc::now();
            if end < now {
                return false;
            }
        }
        true
    }
}
