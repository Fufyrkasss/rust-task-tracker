mod moduls;
use moduls::task::WhatsChanging;
use moduls::task::Task;
use moduls::db::TaskDB;
use moduls::input_text::input;

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
                    Err(e) => println!("ошибка преобразования: {}", e),
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
                    Err(e) => println!("ошибка преобразования: {}", e),
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
                    Err(e) => println!("ошибка преобразования: {}", e),
                }
            },
            "6" => break,
            _ => println!("хз...")
        }
    }
}