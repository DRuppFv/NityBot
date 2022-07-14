use std::{fmt, io::ErrorKind};

use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Lang {
    Pt,
    En,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Lang::*;
        let lang = match *self {
            Pt => "pt",
            En => "en",
        };
        write!(f, "{}", lang)
    }
}

impl TryFrom<String> for Lang {
    type Error = ErrorKind;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        use Lang::*;
        match value.as_str() {
            "pt" => Ok(Pt),
            "en" => Ok(En),
            _ => Err(ErrorKind::Other),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct Guild {
    pub servid: i64,
    pub lang: Lang,
}

impl Guild {
    async fn get_db() -> Pool<Sqlite> {
        sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(sqlx::sqlite::SqliteConnectOptions::new().filename("languages.db"))
            .await
            .expect("")
    }
    pub async fn from_db(id: i64) -> Option<Self> {
        let db = Self::get_db().await;

        if let Ok((servid, lang)) = sqlx::query_as::<_, (i64, String)>(
            "select servid, lang from serverlang where servid = ?",
        )
        .bind(id as i64)
        .fetch_one(&db)
        .await
        {
            if let Ok(lang) = Lang::try_from(lang) {
                Some(Guild { servid, lang })
            } else {
                None
            }
        } else {
            None
        }
    }
    pub async fn new(servid: i64, lang: Lang) -> Self {
        Self { servid, lang }
    }
    pub async fn set_lang(&mut self, lang: Lang) -> Result<SqliteQueryResult, sqlx::Error> {
        self.lang = lang;
        let db = Self::get_db().await;
        sqlx::query("update serverlang set lang = ? where servid = ?")
            .bind(format!("{}", self.lang))
            .bind(self.servid)
            .execute(&db)
            .await
    }

    pub async fn save(&self) -> Result<SqliteQueryResult, sqlx::Error> {
        let db = Self::get_db().await;
        sqlx::query("insert into serverlang (servid, lang) values (?, ?)")
            .bind(self.servid)
            .bind(format!("{}", self.lang))
            .execute(&db)
            .await
    }
}
