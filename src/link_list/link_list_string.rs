use crate::link_list::link_list_string::LinkListNumber::{End, Values};

pub enum LinkListNumber {
    /// 用元祖来存储数据
    Values(u32, Box<LinkListNumber>),
    /// 标记结束
    End,
}

impl LinkListNumber {
    pub(crate) fn new() -> LinkListNumber {
        End
    }

    pub(crate) fn prepend(self, el: u32) -> LinkListNumber {
        Values(el, Box::new(self))
    }

    pub(crate) fn len(&self) -> u32 {
        match *self {
            Values(_, ref next) => 1 + next.len(),
            End => 0,
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match *self {
            Values(value, ref next) => {
                format!("{}, {}", value, next.to_string())
            }
            End => format!("End"),
        }
    }
}

#[allow(dead_code)]
pub fn match_if() {
    let n = Some(-1);
    match n {
        Some(i) if i > 5 => {
            println!("get i is {}", i)
        }
        Some(i) => {
            println!("other i is {:?}", i)
        }
        _ => println!("unknown n"),
    }
}

// noinspection ALL
#[allow(dead_code)]
fn while_let() {
    let mut n = Some(0);
    while let Some(i) = n {
        if i > 5 {
            println!("quit! {}", i);
            n = None;
        } else {
            println!("`i` is {}, try again ", i);
            n = Some(i + 1);
        }
    }
}
