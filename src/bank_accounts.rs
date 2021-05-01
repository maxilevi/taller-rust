use std::thread;
use std::sync::{Arc, Mutex};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }
    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }
    fn balance(&self) -> i32 {
        self.0
    }
}
pub fn main() {
    let account: Account = Account(0);
    let lock = Arc::new(Mutex::new(account));
    let lock1 = lock.clone();
    let lock2 = lock.clone();
    let lock3 = lock.clone();
    let lock4 = lock.clone();
    let customer1_handle = thread::spawn(move || {
        let mut acc = lock1.lock().unwrap();
        acc.deposit(40);
    });
    let customer2_handle = thread::spawn(move || {
        let mut acc = lock2.lock().unwrap();
        acc.withdraw(30);
    });
    let customer3_handle = thread::spawn(move || {
        let mut acc = lock3.lock().unwrap();
        acc.deposit(60);
    });
    let customer4_handle = thread::spawn(move || {
        let mut acc = lock4.lock().unwrap();
        acc.withdraw(70);
    });
    let handles = vec![
        customer1_handle,
        customer2_handle,
        customer3_handle,
        customer4_handle,
    ];
    for handle in handles {
        handle.join().unwrap();
    }
    let acc = lock.lock().unwrap();
    let savings = acc.balance();
    println!("Balance: {:?}", savings);
}
