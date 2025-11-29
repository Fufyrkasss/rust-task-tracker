use super::input_text::input;

pub enum WhatsChanging {
    Nothing,
    Name,
    Info,
    Priority,
    Deadline
}

pub enum TaskPriority {
    Undefined,
    Low,
    Medium,
    High
}

pub enum TaskState {
    Free,
    Taken,
    Completed
}

pub struct Task {
    pub name: String,
    pub info: String,
    pub priority: TaskPriority,
    pub deadline: String,
    pub state: TaskState,
    pub publisher_name: String,
    pub task_worker_name: String
}

impl Task {
    pub fn new() -> Self {
        let new_name = input("имя задачи:");
        let new_info = input("опишите задачу:");
        let input_priority: String = input("приоритет (низкий, средний, высокий):");
        let new_priority: TaskPriority;
        match input_priority.as_str() {
            "низкий" => new_priority = TaskPriority::Low,
            "средний" => new_priority = TaskPriority::Medium,
            "высокий" => new_priority = TaskPriority::High,
            _ => new_priority = TaskPriority::Undefined
        }

        let new_deadline = input("дедлайн (YY:MM:DD:HH:MM):");
        let new_publisher_name = input("имя создателя задачи:");
        
        Self {
            name: new_name,
            info: new_info,
            priority: new_priority,
            deadline: new_deadline,
            state: TaskState::Free,
            publisher_name: new_publisher_name,
            task_worker_name: "None".to_string()
        }
    }
    pub fn new_from_db(parsed_name: String, parsed_info: String, parsed_priority: String, parsed_deadline: String, parsed_state: String, parsed_publisher_name: String, parsed_task_worker_name: String) -> Self {
        let mut new_from_db_priority: TaskPriority = TaskPriority::Undefined;
        let mut new_from_db_state: TaskState = TaskState::Free;
        match parsed_priority.as_str() {
            "низкий" => new_from_db_priority = TaskPriority::Low,
            "средний" => new_from_db_priority = TaskPriority::Medium,
            "высокий" => new_from_db_priority = TaskPriority::High,
            "не задан" => new_from_db_priority = TaskPriority::Undefined,
            _ => println!("ну вот надо мэтчем все обработать")
        }
        match parsed_state.as_str() {
            "не взят" => new_from_db_state = TaskState::Free,
            "взят" => new_from_db_state = TaskState::Taken,
            "завершен" => new_from_db_state = TaskState::Completed,
            _ => println!("ну вот надо мэтчем все обработать")
        }
        Self { name: parsed_name, info: parsed_info, priority: new_from_db_priority, deadline: parsed_deadline, state: new_from_db_state, publisher_name: parsed_publisher_name, task_worker_name: parsed_task_worker_name }
    }
    pub fn get_priority(&self) -> String {
        match self.priority {
            TaskPriority::Undefined => return "не задан".to_string(),
            TaskPriority::Low => return "низкий".to_string(),
            TaskPriority::Medium => return "средний".to_string(),
            TaskPriority::High => return "высокий".to_string()
        }
    }
    pub fn get_state(&self) -> String {
        match self.state {
            TaskState::Free => return "не взят".to_string(),
            TaskState::Taken => return "взят".to_string(),
            TaskState::Completed => return "завершен".to_string()
        }
    }
    pub fn take_task(&mut self) {
        let input_worker_name = input("имя:");
        self.state = TaskState::Taken;
        self.task_worker_name = input_worker_name;
    }
    pub fn complete_task(&mut self) {
        match self.state {
            TaskState::Free => println!("как ты его комплитнул, когда его еще никто не взял?"),
            TaskState::Taken => self.state = TaskState::Completed,
            TaskState::Completed => println!("комплитнуть комплитнутое... жиза")
        }
    }
    fn edit_name(&mut self) {
        self.name = input("введите новое имя:");
    }
    fn edit_info(&mut self) {
        self.info = input("введите новое описание:");
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
        self.deadline = input("введите новый дедлайн (YY:MM:DD:HH:MM):");
    }
    pub fn edit(&mut self) -> WhatsChanging {
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