use std::io;
use rusqlite::{Connection, params, Row};

fn input(text: &str) -> String {
    let mut inp = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut inp)
        .expect("ввел человек какую-то хрень, бывает...");
    return inp.trim().to_string();
}

enum WhatsChanging {
    Nothing,
    Name,
    Info,
    Priority,
    Deadline
}

enum TaskPriority {
    Undefined,
    Low,
    Medium,
    High
}

enum TaskState {
    Free,
    Taken,
    Completed
}

struct TaskDB {
    conn: Connection
}

impl TaskDB {
    fn new() -> Self {
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
            Err(e) => panic!("Ошибка при создании таблицы: {}", e)
        };
        Self { conn: conndb }
    }
    fn parse_tasks(&self) -> Vec<Task> {
        let count: i32 = match self.conn.query_row(
            "SELECT COUNT(*) FROM tasks",
            [],
            |row| row.get(0),
        ) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Ошибка при подсчёте строк: {}", e);
                return Vec::new();
            }
        };

        if count == 0 {
            println!("В таблице нет записей, функция парса не выполняется.");
            return Vec::new();
        }
        let mut stmt = match self.conn.prepare("SELECT id, name, info, priority, deadline, state, publisher_name, task_worker_name FROM tasks") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Ошибка при подготовке запроса: {}", e);
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
                eprintln!("Ошибка при выполнении запроса: {}", e);
                return Vec::new();
            }
        };

        let mut final_vec: Vec<Task> = Vec::new();

        for row in rows_iter {
            match row {
                Ok((name, info, priority, deadline, state, publisher_name, task_worker_name)) => {
                    final_vec.push(Task::new_from_db(name, info, priority, deadline, state, publisher_name, task_worker_name));
                }
                Err(e) => eprintln!("Ошибка при чтении строки: {}", e),
            }
        }
        return final_vec;
    }
    fn insert_task(&self, task: &Task) {
        let sql = "INSERT INTO tasks (name, info, priority, deadline, state, publisher_name, task_worker_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
        match self.conn.execute(sql, params![task.name, task.info, task.get_priority(), task.deadline, task.get_state(), task.publisher_name, task.task_worker_name]) {
            Ok(_) => println!("Таск '{}' добавлен в бд", task.name),
            Err(e) => eprintln!("Ошибка при вставке таска в бд '{}': {}", task.name, e),
        }
    }
    fn update_task_field(&self, id: u16, column: &str, new_value: &str) {
        let sql = format!("UPDATE tasks SET {} = ?1 WHERE id = ?2", column);

        match self.conn.execute(&sql, [new_value, &id.to_string()]) {
            Ok(rows) => {
                if rows == 0 {
                    println!("Таск с id {} не найден.", id);
                } else {
                    println!("Обновлено {} строк(а) для id {}", rows, id);
                }
            }
            Err(e) => eprintln!("Ошибка при обновлении таска: {}", e), // уже 4 часа, до ужаса ******, но раз уж залпом дал, то доделаю уже
        }
    }
}

struct Task {
    name: String,
    info: String,
    priority: TaskPriority,
    deadline: String,
    state: TaskState,
    publisher_name: String,
    task_worker_name: String
}

impl Task {
    fn new() -> Self {
        let n = input("имя задачи:");
        let i = input("опишите задачу:");
        let input_p: String = input("приоритет (низкий, средний, высокий):");
        let p: TaskPriority;
        match input_p.as_str() {
            "низкий" => p = TaskPriority::Low,
            "средний" => p = TaskPriority::Medium,
            "высокий" => p = TaskPriority::High,
            _ => p = TaskPriority::Undefined
        }
        let d = input("дедлайн (YY:MM:DD:HH:MM):"); //Ну уж очень впадлу над временем париться, пощады пж час ночи
        let pn = input("имя создателя задачи:");
        Self {
            name: n,
            info: i,
            priority: p,
            deadline: d,
            state: TaskState::Free,
            publisher_name: pn,
            task_worker_name: "None".to_string()
        }
    }
    fn new_from_db(n: String, i: String, p: String, d: String, s: String, pn: String, twn: String) -> Self {
        let mut pr: TaskPriority = TaskPriority::Undefined;
        let mut st: TaskState = TaskState::Free;
        match p.as_str() {
            "низкий" => pr = TaskPriority::Low,
            "средний" => pr = TaskPriority::Medium,
            "высокий" => pr = TaskPriority::High,
            "не задан" => pr = TaskPriority::Undefined,
            _ => println!("Ну вот надо мэтчем все обработать")
        }
        match s.as_str() {
            "не взят" => st = TaskState::Free,
            "взят" => st = TaskState::Taken,
            "завершен" => st = TaskState::Completed,
            _ => println!("Ну вот надо мэтчем все обработать")
        }
        Self { name: n, info: i, priority: pr, deadline: d, state: st, publisher_name: pn, task_worker_name: twn }
    }
    fn get_priority(&self) -> String {
        match self.priority {
            TaskPriority::Undefined => return "Не задан".to_string(),
            TaskPriority::Low => return "низкий".to_string(),
            TaskPriority::Medium => return "средний".to_string(),
            TaskPriority::High => return "высокий".to_string()
        }
    }
    fn get_state(&self) -> String {
        match self.state {
            TaskState::Free => return "не взят".to_string(),
            TaskState::Taken => return "взят".to_string(),
            TaskState::Completed => return "завершен".to_string()
        }
    }
    fn take_task(&mut self) {
        let worker_name = input("имя:");
        self.state = TaskState::Taken;
        self.task_worker_name = worker_name;
    }
    fn complete_task(&mut self) {
        match self.state {
            TaskState::Free => println!("как ты его комплитнул, когда его еще никто не взял?"),
            TaskState::Taken => self.state = TaskState::Completed,
            TaskState::Completed => println!("комплитнуть комплитнутое... жиза")
        }
    }
    fn edit_name(&mut self) {
        self.name = input("Введите новое имя:");
    }
    fn edit_info(&mut self) {
        self.info = input("Введите новое описание:");
    }
    fn edit_priority(&mut self) {
        let input_p: String = input("приоритет (низкий, средний, высокий):");
            match input_p.as_str() {
                "низкий" => self.priority = TaskPriority::Low,
                "средний" => self.priority = TaskPriority::Medium,
                "высокий" => self.priority = TaskPriority::High,
                _ => self.priority = TaskPriority::Undefined
            }
    }
    fn edit_deadline(&mut self) {
        self.deadline = input("Введите новый дедлайн (YY:MM:DD:HH:MM):");
    }
    fn edit(&mut self) -> WhatsChanging {
        let choice = input("1. Изменить имя\n2. Изменить описание\n3. Изменить приоритет\n4. Изменить дедлайн\n5. Отмена");
        match choice.as_str() {
            "1" => {
                self.edit_name();
                return WhatsChanging::Name;
            },
            "2" => {
                self.edit_info();
                return WhatsChanging::Info;
            },
            "3" => {
                self.edit_priority();
                return WhatsChanging::Priority;
            },
            "4" => {
                self.edit_deadline();
                return WhatsChanging::Deadline;
            },
            "5" => return WhatsChanging::Nothing,
            _ => {
                println!("ну и что это?");
                return WhatsChanging::Nothing;
            }
        }
    }
}

fn main() {
    let taskdb = TaskDB::new();
    let mut tasks: Vec<Task> = taskdb.parse_tasks();
    loop {
        let choice = input("1. Показать все таски\n2. Взять таск\n3. Комплитнуть таск\n4. Создать новый таск\n5. Изменить таск\n6. Выход");
        match choice.as_str() {
            "1" => {
                let mut i = 1;
                for task in &tasks {
                    println!("{}. {}\n{}\n{}\n{}\n{}\n{}\n{}", i, task.name, task.info, task.get_priority(),
                    task.deadline, task.get_state(), task.publisher_name, task.task_worker_name);
                    i += 1;
                }
            }
            "2" => {
                let change = input("номер таска, который хотите взять");
                let number: Result<usize, _> = change.parse::<usize>();
                match number {
                    Ok(num) => {
                        tasks[num - 1].take_task();
                        taskdb.update_task_field(change.parse::<u16>().unwrap(), "task_worker_name", tasks[num - 1].task_worker_name.as_str());
                        taskdb.update_task_field(change.parse::<u16>().unwrap(), "state", tasks[num - 1].get_state().as_str());
                    },
                    Err(e) => println!("Ошибка преобразования: {}", e),
                }
            }
            "3" => {
                let change = input("номер таска, который хотите комплитнуть");
                let number: Result<usize, _> = change.parse::<usize>();
                match number {
                    Ok(num) => {
                        tasks[num - 1].complete_task();
                        taskdb.update_task_field(change.parse::<u16>().unwrap(), "state", tasks[num - 1].get_state().as_str());
                    },
                    Err(e) => println!("Ошибка преобразования: {}", e),
                }
            }
            "4" => {
                let task = Task::new();
                taskdb.insert_task(&task);
                tasks.push(task);
            },
            "5" =>{
                let change = input("номер таска, который хотите изменить:");
                let number: Result<usize, _> = change.parse::<usize>();

                match number {
                    Ok(num) => match tasks[num - 1].edit(){
                        WhatsChanging::Nothing => println!("ничего не изменилось"),
                        WhatsChanging::Name => taskdb.update_task_field(change.parse::<u16>().unwrap(), "name", tasks[num - 1].name.as_str()),
                        WhatsChanging::Info => taskdb.update_task_field(change.parse::<u16>().unwrap(), "info", tasks[num - 1].info.as_str()),
                        WhatsChanging::Priority => taskdb.update_task_field(change.parse::<u16>().unwrap(), "priority", tasks[num - 1].get_priority().as_str()),
                        WhatsChanging::Deadline => taskdb.update_task_field(change.parse::<u16>().unwrap(), "deadline", tasks[num - 1].deadline.as_str())
                    },
                    Err(e) => println!("Ошибка преобразования: {}", e),
                }
            },
            "6" => break,
            _ => println!("хз...")
        }
    }
}