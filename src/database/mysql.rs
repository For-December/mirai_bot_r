use once_cell::sync::OnceCell;

use mysql::{Pool, PooledConn};

static DB_POOL: OnceCell<Pool> = OnceCell::new();

// 初始化数据库链接池
// #[instrument]
pub fn init_mysql_pool(db_url: &str) {
    println!("初始化数据库线程池--------开始-------");
    DB_POOL
        .set(mysql::Pool::new(db_url).expect(&format!("Error connecting to {}", &db_url)))
        .unwrap_or_else(|_| println!("try insert pool cell failure!"));
    println!("初始化数据库线程池--------结束-------");
}

// 从链接链接池里面获取链接
pub fn get_connect() -> PooledConn {
    println!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .expect("Error get_connect from db pool");
    println!("从链接池获取数据库链接----------结束----------");
    conn
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use mysql::prelude::*;
    use serde::{Deserialize, Serialize};

    // use super::{get_connect, init_mysql_pool};

    #[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
    struct User {
        pub id: i64,
        pub name: String,
        pub birth: String,
        pub data: Option<String>,
    }

    #[test]
    pub fn test_mysql() {
        // init_mysql_pool("mysql://root:fy@localhost:3306/test1");
        // let mut conn = get_connect();
        // let res = conn
        //     .query_map("select * from t_user", |(id, name, birth, data)| User {
        //         id,
        //         name,
        //         birth,
        //         data,
        //     })
        //     .expect("query failed...");
        // for ele in res {
        //     println!("{:#?}", ele);
        // }
    }

    #[test]
    pub fn test_mys() {
        // let _url = "mysql://root:fy@localhost:3306/test1";
        // let pool = mysql::Pool::new(_url).unwrap();
        // //连接数据库
        // let mut conn = pool.get_conn().unwrap();
        // let res = conn
        //     .query_map("select * from t_user", |(id, name, birth, data)| User {
        //         id,
        //         name,
        //         birth,
        //         data,
        //     })
        //     .expect("Query failed.");

        // for ele in res {
        //     println!("{:#?}", ele)
        // }
    }
}
