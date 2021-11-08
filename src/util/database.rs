use sqlx::{sqlite::SqlitePool};

pub struct DatabaseTool {
    pub pool: SqlitePool,
}

impl DatabaseTool {
    pub async fn retrieve_str(&self, table: &str, coloumn: &str, id: u64) -> String {
        let row: (String,) = sqlx::query_as(format!("SELECT {} FROM {} WHERE id = {}", coloumn, table, id).as_str())
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database");

        row.0
    }



    pub async fn retrieve_int(&self, table: &str, coloumn: &str, id: u64) -> i64 {
        let row: (i64,) = sqlx::query_as(format!("SELECT {} FROM {} WHERE id = {}", coloumn, table, id).as_str())
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database");

        row.0
    }

    

    pub async fn update(&self, table: &str, coloumn: &str, value: &str, id: u64) {
        let value_scrubbed: String;
        if value.contains("'") {
            value_scrubbed = value.replace("'", "''");
        } else {
            value_scrubbed = String::from(value);
        }
        sqlx::query(format!("UPDATE {} SET {} = '{}' WHERE id = {}", table, coloumn, value_scrubbed, id).as_str())
        .execute(&self.pool)
        .await
        .expect("Could not update database");
    }



    pub async fn lookup(&self, table: &str, id: u64) {
        let row: (bool,) = sqlx::query_as(format!("SELECT EXISTS(SELECT 1 FROM {} WHERE id = {})", table, id).as_str())
        .fetch_one(&self.pool)
        .await
        .expect("Could not query database");

        if !row.0 {
            if table == "guild_settings" {
                sqlx::query(format!("INSERT INTO guild_settings VALUES ({}, '!', '0')", id).as_str())
                .execute(&self.pool)
                .await
                .expect("Could not insert into database");
            }
        }
    }
}
