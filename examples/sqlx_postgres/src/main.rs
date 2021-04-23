use async_std::task;
use sqlx::{PgPool, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, PostgresQueryBuilder, Query, Table};

sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::{bind_query, bind_query_as};

fn main() {

    let connection = task::block_on(async {
        PgPool::connect("postgres://sea:sea@127.0.0.1/query").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    // Schema

    let sql = Table::create()
        .table(Character::Table)
        .if_not_exists()
        .col(ColumnDef::new(Character::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Character::FontSize).integer())
        .col(ColumnDef::new(Character::Character).string())
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        sqlx::query(&sql)
            .execute(&mut pool)
            .await
    });
    println!("Create table character: {:?}\n", result);

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Character, Character::FontSize
        ])
        .values_panic(vec![
            "A".into(),
            12.into(),
        ])
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .execute(&mut pool)
            .await
    });
    println!("Insert into character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = task::block_on(async {
        bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &values)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select one from character:");
    let mut id = None;
    for row in rows.iter() {
        id = Some(row.id);
        println!("{:?}", row);
    }
    let id = id.unwrap();
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![
            (Character::FontSize, 24.into()),
        ])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .execute(&mut pool)
            .await
    });
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = task::block_on(async {
        bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &values)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(PostgresQueryBuilder);

    let row = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .fetch_one(&mut pool)
            .await
            .unwrap()
    });
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &values)
            .execute(&mut pool)
            .await
    });
    println!("Delete character: {:?}", result);
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
}

#[derive(sqlx::FromRow, Debug)]
struct CharacterStruct {
    id: i32,
    character: String,
    font_size: i32,
}
