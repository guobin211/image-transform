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
}
