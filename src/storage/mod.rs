use mysql::prelude::Queryable;

#[derive(Clone)]
pub struct Storage {
    pool: mysql::Pool,
}

impl Storage {
    pub fn new() -> Self {
        let connection_string = Self::get_connection_string();
        let pool = Self::create_pool(&connection_string);
        pool.try_get_conn(10).unwrap();
        log::info!(
            "connected to database at {} successfully.",
            connection_string
        );
        Self { pool }
    }

    fn create_pool(connection_string: &str) -> mysql::Pool {
        mysql::Pool::new(connection_string.to_string()).unwrap()
    }

    fn get_connection_string() -> String {
        let host = std::env::var("MYSQL_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
        let user = std::env::var("MYSQL_USER").unwrap_or_else(|_| "root".to_string());
        let password = std::env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "password".to_string());
        let database =
            std::env::var("MYSQL_DATABASE").unwrap_or_else(|_| "url_shortener".to_string());
        format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        )
    }

    /// save url to database, long term storage
    pub fn save_url(&self, user_id: u64, url: &str, shorten_id: &str) -> Result<u64, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO urls (user_id,target_url,shortened_id) VALUES (?,?,?)",
            (user_id, url, shorten_id),
        )?;
        Ok(conn.last_insert_id())
    }

    /// read url from database, long term storage
    pub fn get_url_by_id(&self, id: u64) -> Result<Option<String>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let url =
            conn.exec_first::<String, &str, _>("SELECT target_url FROM urls WHERE id = ?", (id,))?;
        Ok(url)
    }

    /// read url from database, long term storage
    pub fn get_url_by_shorten_id(
        &self,
        shorten_id: &String,
    ) -> Result<Option<String>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let url = conn.exec_first::<String, &str, _>(
            "SELECT target_url FROM urls WHERE shortened_id = ?",
            (shorten_id,),
        )?;
        Ok(url)
    }
    /// get user based on the username and password
    pub fn get_user(&self, name: &String, password: &String) -> Result<Option<u64>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let user_id = conn.exec_first::<u64, &str, _>(
            "SELECT id FROM users WHERE username = ? AND password = ?",
            (name, password),
        )?;
        Ok(user_id)
    }
}
