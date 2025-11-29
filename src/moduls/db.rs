use rusqlite::{Connection, params, Row};
use super::task::Task;

pub struct TaskDB {
    conn: Connection
}

impl TaskDB {
    pub fn new() -> Self {
        let conndb = match Connection::open("db.sqlite") {
            Ok(c) => c,
            Err(e) => panic!("Ошибка при открытии базы: {}", e)
        };

        let sql = "
            CREATE TABLE IF NOT EXISTS tasks (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                info  INTEGER NOT NULL,
                priority TEXT NOT NULL,
                deadline TEXT NOT NULL,
                state TEXT NOT NULL,
                publisher_name TEXT NOT NULL,
                task_worker_name TEXT NOT NULL
            );
        ";

        match conndb.execute(sql, []) {
            Ok(_) => println!("К работе готовы!"),
            Err(e) => panic!("ошибка при создании таблицы: {}", e)
        };

        Self { conn: conndb }
    }

    pub fn parse_tasks(&self) -> Vec<Task> {
        let count: i32 = match self.conn.query_row(
            "SELECT COUNT(*) FROM tasks",
            [],
            |row| row.get(0),
        ) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("ошибка при подсчёте строк: {}", e);
                return Vec::new();
            }
        };

        if count == 0 {
            println!("в таблице нет записей, функция парса не выполняется.");
            return Vec::new();
        }

        let mut stmt = match self.conn.prepare("SELECT id, name, info, priority, deadline, state, publisher_name, task_worker_name FROM tasks") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ошибка при подготовке запроса: {}", e);
                return Vec::new();
            }
        };

        let rows_iter = match stmt.query_map([], |row: &Row| {
            let name: String = row.get(1)?;
            let info: String = row.get(2)?;
            let priority: String = row.get(3)?;
            let deadline: String = row.get(4)?;
            let state: String = row.get(5)?;
            let publisher_name: String = row.get(6)?;
            let task_worker_name: String = row.get(7)?;
            Ok((name, info, priority, deadline, state, publisher_name, task_worker_name))
        }) {
            Ok(iter) => iter,
            Err(e) => {
                eprintln!("ошибка при выполнении запроса: {}", e);
                return Vec::new();
            }
        };

        let mut final_vec: Vec<Task> = Vec::new();

        for row in rows_iter {
            match row {
                Ok((name, info, priority, deadline, state, publisher_name, task_worker_name)) => {
                    final_vec.push(Task::new_from_db(name, info, priority, deadline, state, publisher_name, task_worker_name));
                }
                Err(e) => eprintln!("ошибка при чтении строки: {}", e),
            }
        }

        return final_vec;
    }
    pub fn insert_task(&self, task: &Task) {
        let sql = "INSERT INTO tasks (name, info, priority, deadline, state, publisher_name, task_worker_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
        match self.conn.execute(sql, params![task.name, task.info, task.get_priority(), task.deadline, task.get_state(), task.publisher_name, task.task_worker_name]) {
            Ok(_) => println!("таск '{}' добавлен в бд", task.name),
            Err(e) => eprintln!("ошибка при вставке таска в бд '{}': {}", task.name, e),
        }
    }

    pub fn update_task_field(&self, id: u16, column: &str, new_value: &str) {
        let sql = format!("UPDATE tasks SET {} = ?1 WHERE id = ?2", column);

        match self.conn.execute(&sql, [new_value, &id.to_string()]) {
            Ok(rows) => {
                if rows == 0 {
                    println!("таск с id {} не найден.", id);
                }
            }
            Err(e) => eprintln!("ошибка при обновлении таска: {}", e)
        }
    }
}