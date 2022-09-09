use std::fmt;

pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{name:{}}}", self.name())
    }
}

pub struct Account {
    user: User,
    value: f64,
}

impl Account {
    pub fn new(user: User, value: f64) -> Self {
        Self { user, value }
    }

    pub fn add_value(&mut self, value: f64) {
        self.value += value;
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_user(&mut self, user: User) {
        self.user = user;
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{user:{}, value:{}}}", self.user(), self.value())
    }
}

pub(crate) fn run() {
    let mut account: Account = Account {
        user: User { name: String::from("Leonardo") },
        value: 100.0,
    };
    println!("{}", account.user.name);
    println!("{}", account.value);

    account.add_value(50.0);
    println!("{}", account.value);

    account.value += 50.0; // só no mesmo arquivo
    println!("{}", account.value);

    println!("{}", account);
}