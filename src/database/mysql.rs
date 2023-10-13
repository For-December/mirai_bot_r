#[cfg(test)]
mod test {

    use std::time::SystemTime;

    use serde::{Deserialize, Serialize};
    use sqlx::{types::time::PrimitiveDateTime, MySqlPool};

    // use super::{get_connect, init_mysql_pool};

    #[derive(Debug, sqlx::FromRow)]
    struct User {
        pub id: Option<i32>,
        pub name: Option<String>,
        pub birth: Option<String>,
        pub data: Option<PrimitiveDateTime>,
    }

    #[test]
    pub fn test_sqlx() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let pool = MySqlPool::connect("mysql://root:fy@localhost:3306/test1")
                    .await
                    .unwrap();
                let user: User = sqlx::query_as!(User, "SELECT * FROM t_user WHERE id = ?", 1)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                println!("{:#?}", user);
            });
    }

    #[test]
    pub fn test_mysql() {
        // init_mysql_pool("");
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
