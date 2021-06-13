use crate::generic_type::trait_type::NewsArticle;

mod trait_type;

struct DPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> DPoint<T, U> {
    #[allow(dead_code)]
    pub fn mix_up<V, W>(self, other: DPoint<V, W>) -> DPoint<T, W> {
        DPoint {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
pub fn run_generic_type() {
    let p1 = DPoint { x: 5.0, y: 6.0 };
    let p2 = DPoint {
        x: "经度",
        y: "纬度",
    };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);

    let p3 = DPoint {
        x: "hello x",
        y: 10,
    };
    let p4 = DPoint {
        x: "hello x",
        y: 100,
    };
    let p5 = p3.mix_up(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
    let news = NewsArticle {
        headline: String::from("标题"),
        location: String::from("china"),
        author: String::from("michael"),
        content: String::from("new content"),
    };
    println!("news is {:?}", news);
}

#[allow(dead_code)]
pub fn life_time_code<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
pub fn pure_code(x: Box<str>, y: Box<str>) -> Box<str> {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
