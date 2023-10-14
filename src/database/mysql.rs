use serde_json::Value;
use sqlx::{types::time::PrimitiveDateTime, FromRow, MySqlPool};

use crate::bot::message::Message;

#[derive(Debug, FromRow, Default)]
struct AskAnswer {
    pub id: Option<i32>,
    pub asker_id: Option<String>,
    pub replier_id: Option<String>,
    pub ask_text: Option<String>,
    pub answer: Option<Value>,
    pub update_time: Option<PrimitiveDateTime>,
    pub create_time: Option<PrimitiveDateTime>,
}
fn get_url() -> String {
    let database_url: Option<String> = (|| -> Option<String> {
        for ele in dotenvy::dotenv_iter().expect("读取.env 文件失败") {
            let (key, val) = ele.expect("未能解包字段");
            if key.eq("DATABASE_URL") {
                return Some(val);
            }
        }
        return None;
    })();
    database_url.expect("请配置DATABASE_URL")
}
pub fn get_nearest_answer(ask: &str) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let database_url = get_url();
    let res = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // println!("{}",m);
            let pool = MySqlPool::connect(&database_url).await.unwrap();
            // 原来之前的报错是返回值类型不匹配啊，没有解包
            let res: AskAnswer = sqlx::query_as!(
            AskAnswer,
             "SELECT id,asker_id,replier_id,ask_text, answer,create_time,update_time FROM ask_answer WHERE LEVENSHTEIN(?,ask_text) < 3 ORDER BY LEVENSHTEIN(?,ask_text) ASC", // ascending&descending
             ask,ask)
            .fetch_one(&pool)
            .await
            .unwrap();

            let res: Vec<Message> = serde_json::from_value(res.answer.unwrap())
                .unwrap_or_else(|_| panic!("解析messageChain失败"));
            res
        });
    println!("{:#?}", res);
    Ok(res)
}

pub fn set_ask_answer(ask: &str, asker_id: &str, replier_id: &str, answer: Vec<Message>) {
    let answer = serde_json::to_value(answer).unwrap();
    let database_url = get_url();
    let res = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // println!("{}",m);
            let pool = MySqlPool::connect(&database_url).await.unwrap();
            // 原来之前的报错是返回值类型不匹配啊，没有解包
            sqlx::query!(
                "INSERT INTO ask_answer(asker_id,replier_id,ask_text,answer) VALUES(?,?,?,?)",
                asker_id,
                replier_id,
                ask,
                answer,
            )
            .execute(&pool)
            .await
            .expect("添加失败！");
        });
    println!("{:#?}", res);
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

    use super::{get_nearest_answer, AskAnswer};

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
        let res = get_nearest_answer("测试").unwrap();
        println!("{:#?}", res);
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
