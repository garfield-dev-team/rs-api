use crate::core::constants;

#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: i32, name: String, price: f64) -> Self {
        Self { id, name, price }
    }

    // 加购
    pub fn add_cart(&self) {
        println!("{} added to cart.", self.name);
        panic!("{}", constants::ADD_CART_COUNT)
    }

    // 领券
    pub fn receive_coupon(&mut self, discount: f64) {
        self.price *= discount;
        println!("{}", constants::RECEIVE_COUPON);
    }

    // 凑单
    pub fn add_up(&self) {
        println!("{} added up.", self.name);
    }

    // 结算
    pub fn checkout(&self) {
        println!("{} checkout.", self.name);
    }
}
