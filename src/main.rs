use crate::structs::Account;
use crate::structs::User;

mod error;
mod conditional;
mod loops;
mod memory;
mod structs;

fn main() {
    // structs::run();
    let user = User::new(String::from("Leonardo"));
    let account = Account::new(user, 54.0);

    println!("{}", account);
    let vo = account.to_vo();
    println!("{}, {}", vo.name, vo.value);
    println!("{}", account);
}

impl Account {
    pub fn to_vo(&self) -> AccountVo {
        return AccountVo {
            name: Box::from(self.user().name()),
            value: Box::from(self.value()),
        };
    }
}

pub struct AccountVo {
    pub name: Box<str>,
    pub value: Box<f64>,
}
