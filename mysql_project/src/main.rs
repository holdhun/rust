use mysql::*;
use mysql::prelude::*;
use std::panic;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name:String,
}

fn main() {
    let dsn = String::from("mysql://root:root@127.0.0.1:3306/test");
    let pool = Pool::new(dsn).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //数据库  user(id,name)

    //insert
    {
        let ret = "insert into user(name,pwd)
        values(?,?)".with((1,2)).run(&mut conn).unwrap();
        println!("insert id:{:?}",ret.last_insert_id().unwrap());
    }

    //delete
    {
        
        let ret = "DELETE FROM `user` WHERE id=? and ?".with((1,true)).run(&mut conn).unwrap();
        println!("affect row:{:?}",ret.affected_rows());
    }
    //update
    {
        
        let ret = "update `user` set name=? WHERE id=?".with(("test5",5)).run(&mut conn).unwrap();
        println!("affect row:{:?}",ret.affected_rows());
    }
    //select
    {
        
        let ret = "select id,name from user".map(&mut conn,|(id,name)|{
            User{
                id:id,
                name:name,
            }
        }).unwrap();
        println!("{:?}",ret);
    }
    //transaction 事务
    {
        let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
        let insertId1 =  {
            "insert into user(name,pwd)values(?,?)".with(("","")).run(&mut tx).unwrap().last_insert_id().unwrap()
        };
        println!("insertid1:{:?}",insertId1);
        let update2 = {
            "update `user` set name=? WHERE id=?".with(("hello test",insertId1)).run(&mut tx).unwrap().affected_rows()
        };
        println!("insertid2:{:?}",update2);

        if insertId1>0 && update2>0 {
            tx.commit();
        }else{
            tx.rollback();
        }

    }



}
