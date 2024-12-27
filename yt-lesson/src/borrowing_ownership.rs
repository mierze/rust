use std::u64;

fn main() {
    // borrowing and referencing
    println!("Hello world!");
    let mut _x: i32 = 5;
    // let _r: &mut i32 = &_x;
    // *_r += 1;
    // println!("_x {}, _r {}", _x, _r);

    let mut bank: BankAccount = BankAccount {
        owner: "brett".to_string(),
        balance: 149.00,
    };
    bank.withdraw(22.99);
}

struct BankAccount {
    owner: String,
    balance: f64,
}
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "withdrawing {} from account owned by {}",
            amount, self.owner
        );
    }
}
