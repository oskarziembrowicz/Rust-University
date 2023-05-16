#[derive(Debug, PartialEq)]

struct Point {
    x: f32,
    y: f32,
}
#[derive(PartialEq)]
enum Axis {
    ox,
    oy,
}

enum RefactorFunction {
    move_by_vector(f32, f32),
    scale(f32),
    rotate_by_degrees(f32),
    mirror_by_axis(Axis),
}

impl Point {

    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn move_by_vector(&mut self, mx: f32, my: f32) -> () {
        self.x += mx;
        self.y += my
    }

    fn scale(&mut self, scale: f32) -> () {
        self.x *= scale;
        self.y *= scale
    }

    fn rotate_by_degrees(&mut self, degrees: f32) -> () {
        self.x = self.x * degrees.cos() - self.y * degrees.sin();
        self.y = self.x * degrees.sin() - self.y * degrees.cos();
    }

    fn mirror_by_axis(&mut self, axis: Axis) -> Option<()> {
        if axis == Axis::ox {
            self.y *= -1.0;
            return Some(());
        }
        else if axis == Axis::oy {
            self.x *= -1.0;
            return Some(());
        }
        None
    }

    fn refactor(&mut self, function: RefactorFunction) -> Option<()> {

        match function {
            RefactorFunction::rotate_by_degrees(deg) => {
                self.rotate_by_degrees(deg); Some(())
            },
            RefactorFunction::move_by_vector(x, y) => {
                self.move_by_vector(x, y); Some(())
            },
            RefactorFunction::scale(s) => {
                self.scale(s); Some(())
            },
            RefactorFunction::mirror_by_axis(axis) => {
                self.mirror_by_axis(axis); Some(())
            }
        };
        None
    }

    fn distance_from_00(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

}

fn main() {
    let mut point1 = Point::new(2.0, 4.0);
    point1.move_by_vector(-1.0, 3.0);
    println!("{:?}", point1);
    point1.scale(2.0);
    println!("{:?}", point1);
    point1.rotate_by_degrees(30.0);
    println!("{:?}", point1);
    point1.mirror_by_axis(Axis::ox);
    println!("{:?}", point1);
    point1.refactor(RefactorFunction::mirror_by_axis(Axis::oy));
    println!("{:?}", point1);
    println!("{:?}", point1.distance_from_00());
}
