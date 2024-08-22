use rusqlite::{Connection, Result};

use crate::models::Task; 

pub fn connect() -> Result<Connection> {
    let conn: Connection = Connection::open("tasks.db")?;

    let _ = conn.execute(
        "
            CREATE TABLE IF NOT EXISTS tasks(
                id INTEGER PRIMARY KEY, 
                name TEXT NOT NULL, 
                description TEXT NOT NULL, 
                is_complete BOOLEAN NOT NULL
            ); 
        ",
        (),
    );

    Ok(conn)
}

pub fn fetch_tasks() -> Result<Vec<Task>> {
    let conn: Connection = connect()?;

    let mut stmt = conn.prepare("SELECT * FROM tasks")?;

    let tasks_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            is_complete: row.get(3)?,
        })
    })?;

    let tasks: Vec<Task> = tasks_iter.collect::<Result<_, rusqlite::Error>>()?;

    Ok(tasks)
}

pub fn get_single_task(task_id: u32) -> Result<Option<Task>> {
    let conn = connect()?;

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;

    let task_iter = stmt.query_map([task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            is_complete: row.get(3)?,
        })
    })?;

    let task: Option<Task>;

    let selected_task: Vec<Task> = task_iter.collect::<Result<_, rusqlite::Error>>()?;

    task = match selected_task.get(0) {
        Some(sel_task) => Some(sel_task.clone()),
        None => None,
    };

    Ok(task)
}

pub fn create_task<'a>(task: &'a Task) -> Result<()> {
    let conn = connect()?;

    let Task {
        id,
        name,
        description,
        is_complete,
    } = task;

    conn.execute(
        "INSERT INTO tasks(name, description, is_complete) VALUES(?1, ?2, ?3)",
        (name, description, is_complete),
    )?;

    Ok(())
}

pub fn delete_task(task_id: u32) -> Result<()> {
    let conn: Connection = connect()?;

    conn.execute(
        format!("DELETE FROM tasks WHERE id = {}", task_id).as_str(),
        (),
    )?;

    Ok(())
}

pub fn set_task_is_complete(task_id: u32, is_complete: bool) -> Result<()> {
    let conn: Connection = connect()?;

    conn.execute(
        format!(
            "UPDATE tasks SET is_complete = {} WHERE id = {}",
            is_complete, task_id
        )
        .as_str(),
        (),
    )?;

    Ok(())
}

pub fn update_task<'a>(task_id: u32, task: &'a Task) -> Result<bool> {
    // Initialize connection
    let conn: Connection = connect()?;

    // First get requested task
    let update_status: bool;
    let selected_task: Option<Task> = get_single_task(task_id)?;

    if let Some(_) = selected_task {
        // Do update operation here
        let _ = conn.execute(
            "UPDATE tasks SET name = ?1, description = ?2, is_complete = ?3",
            (
                task.name.clone(),
                task.description.clone(),
                task.is_complete,
            ),
        );

        update_status = true;
    } else {
        update_status = false;
    }

    Ok(update_status)
}
