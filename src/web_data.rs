use std::cell::RefCell;


//线程安全的变量，方便会话调用
thread_local!(static USERNAME: RefCell<String> = RefCell::new("".to_string()));




pub fn set_user_name(user_name: String) {
    USERNAME.with(|f| {
        *f.borrow_mut() = user_name;
    });
}

pub fn get_user_name() -> String {
    // we retain our original value of 2 despite the child thread
    let mut user_name = "".to_string();
    USERNAME.with(|f| {
        user_name = f.borrow().to_string()
    });
    user_name
}



