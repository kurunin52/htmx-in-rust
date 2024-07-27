use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub completed_on: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get(pool: &PgPool, id: &i32) -> Todo {
    sqlx::query_as("SELECT id, content, completed_on FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

pub async fn get_list(pool: &PgPool) -> Vec<Todo> {
    sqlx::query_as("SELECT id, content, completed_on FROM todos ORDER BY id DESC")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn create(pool: &PgPool, content: &String) -> Todo {
    sqlx::query_as("INSERT INTO todos(content) VALUES ($1) RETURNING id, content, completed_on")
        .bind(content)
        .fetch_one(pool)
        .await
        .unwrap()
}

pub async fn update_as_done(pool: &PgPool, id: &i32) -> Result<()> {
    let result = sqlx::query("UPDATE todos SET completed_on = $2 WHERE id = $1")
        .bind(id)
        .bind(Utc::now())
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn update(pool: &PgPool, todo: &Todo) -> Result<()> {
    let result = sqlx::query("UPDATE todos SET content = $2, completed_on = $3 WHERE id = $1")
        .bind(todo.id)
        .bind(&todo.content)
        .bind(todo.completed_on)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn delete(pool: &PgPool, id: &i32) -> Result<()> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
