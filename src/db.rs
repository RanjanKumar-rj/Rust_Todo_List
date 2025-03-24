use rusqlite::{Connection, Result, params};

pub fn create_db() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    println!("Database and table created successfully!");
    Ok(conn)
}

pub fn insert_task(task: &str) -> Result<()> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "INSERT INTO todo (task, done) VALUES (?1, 0)", // Default done = 0 (not completed)
        params![task],
    )?;

    println!("Task added successfully!");
    Ok(())
}

pub fn fetch_tasks() -> Result<()> {
    let conn = Connection::open("todo.db")?;
    let mut stmt = conn.prepare("SELECT id, task, done FROM todo")?;
    let tasks_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
        ))
    })?;

    for task in tasks_iter {
        let (id, task, done) = task?;
        let status = if done == 1 {
            "✔ Completed"
        } else {
            "❌ Not Done"
        };
        println!("ID: {}, Task: {}, Status: {}", id, task, status);
    }

    Ok(())
}

pub fn mark_task_done(id: i32) -> Result<()> {
    let conn = Connection::open("todo.db")?;
    conn.execute("UPDATE todo SET done = 1 WHERE id = ?1", params![id])?;

    Ok(())
}

pub fn delete_task(id: i32) -> Result<()> {
    let conn = Connection::open("todo.db")?;
    conn.execute("DELETE FROM todo WHERE id = ?1", params![id])?;

    Ok(())
}

// pub fn update_task(id: i32, new_task: &str, done: bool) -> Result<()> {
//     let conn = Connection::open("todo.db")?;
//     let done_value = if done { 1 } else { 0 }; // Convert `bool` to `INTEGER`

//     conn.execute(
//         "UPDATE todo SET task = ?1, done = ?2 WHERE id = ?3",
//         params![new_task, done_value, id],
//     )?;

//     println!("Task updated successfully!");
//     Ok(())
// }
