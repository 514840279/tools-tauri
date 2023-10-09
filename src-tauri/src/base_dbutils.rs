use mysql::*;
use mysql::prelude::*;

use once_cell::sync::OnceCell;
use tracing::{instrument, info,error};
use chrono::NaiveDateTime;
use serde::Serialize;

// 创建一个全局的DB_POOL，可以一直使用，启动的时候初始化即可
static DB_POOL: OnceCell<Pool> = OnceCell::new();

pub struct User
{
    pub id: String,
    pub name: String,
    pub age: i32,
}

pub fn conn_mysql()->PooledConn{
    //设置连接字符串 localhost 注意端口:3306
    let _url="mysql://root:root@localhost:3306/application";
    let pool=Pool::new(_url).unwrap();
    //连接数据库
    let conn=pool.get_conn().unwrap();
    return conn;
}

pub fn select(sqlstr: &str)->User{
    //连接数据库
    let mut conn=conn_mysql();
    let mut user=User{id:String::from("r.1"),name:String::from(""), age:0};
    conn.query_iter(sqlstr).unwrap()
    .for_each(|row|{
        let r:(String,String,i32)=from_row(row.unwrap());
        user=User{id:String::from(r.0),name:String::from(r.1), age:r.2};
    });
    
    return user;
}

// 初始化数据库链接池
#[instrument]
pub fn init_mysql_pool(db_url: &str) {
    info!("初始化数据库线程池--------开始-------");
    DB_POOL.set(mysql::Pool::new(&db_url).expect(&format!("Error connecting to {}", &db_url)))
        .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    info!("初始化数据库线程池--------结束-------");
}
// 从链接链接池里面获取链接
#[instrument]
pub fn get_connect() -> PooledConn {
    info!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL.get().expect("Error get pool from OneCell<Pool>").get_conn().expect("Error get_connect from db pool");
    info!("从链接池获取数据库链接----------结束----------");
    conn
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub account: String,
    pub password: String,
    pub enabled: i32,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}


pub fn get_by_id(id: &str) -> Option<Account> {
    info!("从链接池获取数据库链接----------结束----------");
    // 获取数据库链接
    let mut conn = get_connect();
    // 根据id查询账号信息
    let query_result = conn.exec_first("select id,account,password,enabled,create_time,modify_time from account where id=:id", params!("id"=>id))
        .map(|row| {
            row.map(|(id, account, password, enabled, create_time, modify_time)| Account { id, account, password, enabled, create_time, modify_time })
        });
    // 判断是否查询到数据
    match query_result {
        Ok(result) => {
            info!("从链接池获取数据库链接----------结束----------");
            result
        }
        Err(_) => {
            error!("异常DDD");
            None
        }
    }
}