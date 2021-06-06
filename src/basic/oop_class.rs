#[derive(Debug)]
#[allow(dead_code)]
pub enum Sex {
    Man,
    Women,
}

/// class
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    sex: Sex,
    phone: String,
}

/// 接口
#[allow(dead_code)]
trait HasVoiceBox {
    fn speck(&self);
    fn can_speck(&self) -> bool;
}

/// 实现接口
impl HasVoiceBox for Person {
    fn speck(&self) {
        println!("my name is {0}, age is  {1}", self.name, self.age);
    }

    fn can_speck(&self) -> bool {
        return self.age > 1;
    }
}

/// 实现class
#[allow(dead_code)]
impl Person {
    pub(crate) fn new(name: &str, age: u32, sex: Sex, phone: &str) -> Person {
        Person {
            name: name.to_string(),
            age,
            sex,
            phone: phone.to_string(),
        }
    }

    pub(crate) fn crate_by_name(name: &str) -> Person {
        Person {
            name: name.to_string(),
            age: 1,
            sex: Sex::Women,
            phone: "".to_string(),
        }
    }

    pub(crate) fn is_safe_years(&self) -> bool {
        return self.age >= 18;
    }

    pub(crate) fn log_description(&self) {
        match &self.sex {
            Sex::Man => {
                println!("{} is man", &self.name);
            }
            Sex::Women => {
                println!("{} is woman", &self.name);
            }
        }
    }
}
