use serde_json::Value;
use sqlx::{
    types::{time::PrimitiveDateTime, Json},
    FromRow,
};

use crate::bot::message::Message;

#[derive(Debug, FromRow)]
struct AskAnswer {
    pub id: Option<i32>,
    pub asker_id: Option<String>,
    pub replier_id: Option<String>,
    pub ask_text: Option<String>,
    pub answer: Option<Value>,
    pub update_time: Option<PrimitiveDateTime>,
    pub create_time: Option<PrimitiveDateTime>,
}

#[cfg(test)]
mod test {
    use std::path::Ancestors;

    use crate::bot::message::Message;
    use serde_json::Value;

    use sqlx::{
        types::{time::PrimitiveDateTime, Json},
        MySqlPool,
    };

    use super::AskAnswer;

    // use super::{get_connect, init_mysql_pool};

    #[derive(Debug, sqlx::FromRow)]
    struct User {
        pub id: Option<i32>,
        pub name: Option<String>,
        pub birth: Option<String>,
        pub data: Option<PrimitiveDateTime>,
    }

    #[test]
    pub fn test_ask_answer() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let pool = MySqlPool::connect("mysql://root:fy@localhost:3306/group_msg_map")
                    .await
                    .unwrap();
                // 原来之前的报错是返回值类型不匹配啊，没有解包
                    let res:AskAnswer = sqlx::query_as!(AskAnswer, "SELECT id,asker_id,replier_id,ask_text, answer,create_time,update_time FROM ask_answer WHERE id = 1").fetch_one(&pool).await.unwrap();
                    
                    
                    let res:Message = serde_json::from_value(res.answer.unwrap()).unwrap();

                    println!("{:#?}",res)
            });
    }

    #[test]
    pub fn test_sqlx() {
        // tokio::runtime::Builder::new_multi_thread()
        // .enable_all()
        // .build()
        // .unwrap()
        // .block_on(async {
        //     let pool = MySqlPool::connect("mysql://root:fy@localhost:3306/test1")
        //         .await
        //         .unwrap();
        //     let user: User = sqlx::query_as!(
        //         User,
        //         "SELECT * FROM t_user WHERE LEVENSHTEIN(?,name) < 3 ",
        //         "jack"
        //     )
        //     .fetch_one(&pool)
        //     .await
        //     .unwrap();
        //     println!("{:#?}", user);
        // });
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
