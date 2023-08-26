use core::fmt;
use indexmap::IndexMap;
use poise::serenity_prelude::{GuildId, RoleId, UserId};
use sqlx::{MySqlPool, Row};
use std::{env, fmt::Debug};

#[derive(Clone)]
pub struct DatabaseManager {
    pub pool: MySqlPool,
}

pub trait ValidValue: 'static {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Display for dyn ValidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vv_fmt(f)
    }
}

impl ValidValue for UserId {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidValue for RoleId {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidValue for GuildId {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidValue for u64 {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ValidValue for u128 {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ValidValue for &'static str {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contains("'") {
            let scrubbed: String;
            scrubbed = self.replace("'", "''");
            write!(f, "{}", scrubbed)
        } else {
            write!(f, "{}", self)
        }
    }
}

impl ValidValue for bool {
    fn vv_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bool_to_str(*self))
    }
}

fn bool_to_str(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

pub trait ValidID: 'static {
    fn to_u64(&self) -> u64;
    fn vi_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Display for dyn ValidID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vi_fmt(f)
    }
}

impl ValidID for UserId {
    fn to_u64(&self) -> u64 {
        self.0
    }

    fn vi_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidID for RoleId {
    fn to_u64(&self) -> u64 {
        self.0
    }

    fn vi_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValidID for GuildId {
    fn to_u64(&self) -> u64 {
        self.0
    }

    fn vi_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ColumnType {
    which: &'static str,
}

pub const TEXT: ColumnType = ColumnType { which: "TEXT" };
pub const INTEGER: ColumnType = ColumnType { which: "BIGINT" };
pub const BOOL: ColumnType = ColumnType { which: "BOOLEAN" };

impl DatabaseManager {
    pub async fn retrieve_str<T: ValidID>(
        &self,
        table: &str,
        retrive_column: &str,
        where_column: &str,
        where_value: &T,
    ) -> Option<String> {
        let row: (Option<String>,) = sqlx::query_as(
            format!(
                "SELECT {} FROM {} WHERE {} = {}",
                retrive_column,
                table,
                where_column,
                where_value.to_u64(),
            )
            .as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database [retrieve_str]");

        row.0
    }

    pub async fn retrieve_int<T: ValidID>(
        &self,
        table: &str,
        retrive_column: &str,
        where_column: &str,
        where_value: &T,
    ) -> i64 {
        let row: (Option<i64>,) = sqlx::query_as(
            format!(
                "SELECT {} FROM {} WHERE {} = {}",
                retrive_column,
                table,
                where_column,
                where_value.to_u64()
            )
            .as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database [retrieve_int]");

        row.0.unwrap_or(0)
    }

    pub async fn retrieve_bool<T: ValidID>(
        &self,
        table: &str,
        retrive_column: &str,
        where_column: &str,
        where_value: &T,
    ) -> Option<bool> {
        let row: (Option<bool>,) = sqlx::query_as(
            format!(
                "SELECT {} FROM {} WHERE {} = {}",
                retrive_column,
                table,
                where_column,
                where_value.to_u64()
            )
            .as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database [retrieve_int]");

        row.0
    }

    // pub async fn update_str<T: ValidID>(
    //     &self,
    //     table: &str,
    //     update_column: &str,
    //     update_str: &str,
    //     where_column: &str,
    //     where_value: &T,
    // ) {
    //     let update_str_scrubbed: String;
    //     if update_str.contains("'") {
    //         update_str_scrubbed = update_str.replace("'", "''");
    //     } else {
    //         update_str_scrubbed = String::from(update_str);
    //     }
    //     sqlx::query(
    //         format!(
    //             "UPDATE {} SET {} = '{}' WHERE {} = {}",
    //             table,
    //             update_column,
    //             update_str_scrubbed,
    //             where_column,
    //             where_value.as_u64()
    //         )
    //         .as_str(),
    //     )
    //     .execute(&self.pool)
    //     .await
    //     .expect("Could not update database [update]");
    // }

    // pub async fn update_int<T: ValidValue, U: ValidID>(
    //     &self,
    //     table: &str,
    //     update_column: &str,
    //     update_int: &T,
    //     where_column: &str,
    //     where_value: &U,
    // ) {
    //     sqlx::query(
    //         format!(
    //             "UPDATE {} SET {} = {} WHERE {} = {}",
    //             table,
    //             update_column,
    //             update_int.to_str(),
    //             where_column,
    //             where_value.as_u64()
    //         )
    //         .as_str(),
    //     )
    //     .execute(&self.pool)
    //     .await
    //     .expect("Could not update database [update]");
    // }

    // pub async fn update_bool<U: ValidID>(
    //     &self,
    //     table: &str,
    //     update_column: &str,
    //     update_bool: bool,
    //     where_column: &str,
    //     where_value: &U,
    // ) {
    //     sqlx::query(
    //         format!(
    //             "UPDATE {} SET {} = {} WHERE {} = {}",
    //             table,
    //             update_column,
    //             bool_to_str(update_bool),
    //             where_column,
    //             where_value.as_u64()
    //         )
    //         .as_str(),
    //     )
    //     .execute(&self.pool)
    //     .await
    //     .expect("Could not update database [update]");
    // }

    pub async fn update_value<T: ValidValue + fmt::Display, U: ValidID + fmt::Display>(
        &self,
        table: &str,
        keys: Vec<&str>,
        types: Vec<ColumnType>,
        default_values: Vec<&str>,
        update_column: &str,
        update_value: &T,
        where_column: &str,
        where_id: &U,
    ) {
        // Create table if it does not exist
        if !self.table_exists(table).await {
            self.create_table(table, &keys, &types).await;
        }

        // Populate rows with default values if it does not exist
        if !self.row_exists(table, where_column, where_id).await {
            let mut default_values = default_values;
            let id = format!("{}", where_id);
            default_values[0] = &id;
            self.insert_row(table, &default_values).await;
        }

        sqlx::query(
            format!(
                "UPDATE {} SET {} = '{}' WHERE {} = {}",
                table, update_column, update_value, where_column, where_id,
            )
            .as_str(),
        )
        .execute(&self.pool)
        .await
        .expect("Could not update database [update]");
    }

    /// Returns true if a row exists in a given table, with a given value, at a given column
    pub async fn row_exists<T: ValidID + fmt::Display>(
        &self,
        table: &str,
        column_name: &str,
        id: &T,
    ) -> bool {
        // let row: (bool,) = sqlx::query_as(format!("SELECT EXISTS(SELECT 1 FROM {} WHERE {} = '{}')", table, column_name, value).as_str())
        let row: (bool,) = sqlx::query_as(
            format!(
                "SELECT EXISTS(SELECT {} FROM {} WHERE {} = '{}')",
                column_name, table, column_name, id
            )
            .as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database [row_exists]");

        row.0
    }

    // Should probably increase safety on this at some point
    pub async fn insert_row(&self, table: &str, values: &[&str]) {
        let mut qry = format!("INSERT INTO {} VALUES (", table);

        for s in values {
            match s.parse::<i64>() {
                Ok(i) => {
                    qry.push_str(&format!("{}, ", i));
                    continue;
                }
                _ => (),
            }

            match s.parse::<bool>() {
                Ok(b) => {
                    if b {
                        qry.push_str("1, ");
                    } else {
                        qry.push_str("0, ");
                    }
                    continue;
                }
                _ => (),
            }

            qry.push_str(&format!("'{}', ", s));
        }

        qry.pop();
        qry.pop();
        qry.push(')');

        sqlx::query(&qry)
            .execute(&self.pool)
            .await
            .expect("Could not insert into database [insert_row 3]");
    }

    pub async fn delete_row<T: ValidValue + fmt::Display>(
        &self,
        table: &str,
        column_name: &str,
        value: &T,
    ) {
        sqlx::query(format!("DELETE FROM {} WHERE {} = {}", table, column_name, value).as_str())
            .execute(&self.pool)
            .await
            .expect("Could not update database [update]");
    }

    pub async fn table_exists(&self, table: &str) -> bool {
        let row: (bool,) = sqlx::query_as(format!("SELECT EXISTS(SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_name = '{}')", env::var("MYSQL_DB").unwrap(), table).as_str())
            .fetch_one(&self.pool)
            .await
            .expect("Could not query database [table_exists]");

        row.0
    }

    // Should probably increase safety on this at some point
    pub async fn create_table(&self, table: &str, keys: &[&str], types: &[ColumnType]) {
        // Make sure there's a key for every declared type
        let keys_len = keys.len();
        if keys_len != types.len() {
            panic!(
                "Mismatch between amount of keys and types provided\nkeys: {:?}\ntypes: {:?}",
                keys, types
            )
        }

        // Create the query for the database base on how many columns were provided
        let mut qry = format!("CREATE table IF NOT exists {}(", table);
        let last_index = keys_len - 1;
        if keys_len >= 2 {
            for (i, k) in keys[..last_index].into_iter().enumerate() {
                qry = format!("{}{} {}, ", qry, k, types[i].which)
            }
        }
        qry = format!("{}{} {})", qry, keys[last_index], types[last_index].which);

        sqlx::query(&qry)
            .execute(&self.pool)
            .await
            .expect("Could not create table [create_table]");
    }

    pub async fn get_all_rows(&self, table: &str, key: &str) -> Vec<u64> {
        let result = sqlx::query(format!("SELECT {} FROM {}", key, table).as_str())
            .fetch_all(&self.pool)
            .await
            .expect("Could not query database [get_all_rows]");

        result
            .iter()
            .map(|i| i.get::<i64, usize>(0) as u64)
            .collect()
    }

    pub async fn get_rows_sorted(
        &self,
        table: &str,
        key: &str,
        sort_key: &str,
        lower_bound: u64,
        rows: u64,
    ) -> IndexMap<u64, u64> {
        let result = sqlx::query(
            format!(
                "SELECT {},{} FROM {} ORDER BY {} DESC LIMIT {},{}",
                key, sort_key, table, sort_key, lower_bound, rows
            )
            .as_str(),
        )
        .fetch_all(&self.pool)
        .await
        .expect("Could not query database [get_rows_sorted]");

        result
            .iter()
            .map(|row| {
                (
                    row.get::<i64, usize>(0) as u64,
                    row.get::<i64, usize>(1) as u64,
                )
            })
            .collect()
    }
}
