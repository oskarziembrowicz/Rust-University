#[derive(Debug)]

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {

    fn from_3u8(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb {
            r: red,
            g: green,
            b: blue,
        }
    }

    fn from_html(code: String) -> Option<Rgb> {
        let mut counter = 0;
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut n3 = String::new();
        for i in code.chars() {
            if i == '#' {
                continue;
            }
            if counter == 0 || counter == 1 {
                n1.push(i);
            } else if counter == 2 || counter == 3 {
                n2.push(i);
            } else if counter == 4 || counter == 5 {
                n3.push(i);
            } else {
                return None;
            }
            counter += 1;
        }

        Some(Rgb::from_3u8(u8::from_str_radix(&n1, 16).unwrap(), u8::from_str_radix(&n2, 16).unwrap(), u8::from_str_radix(&n3, 16).unwrap()))
    }

    fn from_3f32(n1: f32, n2: f32, n3: f32) -> Option<Rgb> {

        if n1 > 1.0 || n2 > 1.0 || n3 > 1.0 {
            return None;
        }

        let red = (255.0 * n1) as u8;
        let green = (255.0 * n2) as u8;
        let blue = (255.0 * n3) as u8;

        Some(Rgb::from_3u8(red, green, blue))
    }

    fn black() -> Rgb {
        Rgb::from_3u8(0,0,0)
    }

    fn white() -> Rgb {
        Rgb::from_3u8(255,255,255)
    }

    fn red() -> Rgb {
        Rgb::from_3u8(255,0,0)
    }

    fn green() -> Rgb {
        Rgb::from_3u8(0,255,0)
    }

    fn blue() -> Rgb {
        Rgb::from_3u8(0,0,255)
    }

    fn as_3u8(self) -> (u8, u8, u8){
        (self.r, self.g, self.b)
    }

    fn as_3f32(self) -> (f32, f32, f32){
        ((self.r as f32) / 255.0, (self.g as f32) / 255.0,(self.b as f32) / 255.0)
    }

    fn brightness(self) -> f32 {
        let colors = self.as_3f32();
        (colors.0 + colors.1 + colors.2) / 3.0
    }

    fn make_darker(&mut self, k: f32) -> Option<()> {
        if k > 1.0 {
            return None;
        }
        self.r = ((self.r as f32) * k) as u8;
        self.g = ((self.g as f32) * k) as u8;
        self.b = ((self.r as f32) * k) as u8;
        Some(())
    }
}

fn main() {
    let color1 = Rgb::from_3u8(128, 128, 128);
    let mut color2 = Rgb::from_html("ffffff".to_string()).unwrap();
    let mut color3 = Rgb::from_3f32(0.8,0.9,1.0).unwrap();
    println!("{:?}", color1.brightness());
    println!("{:?}", color2.brightness());
    println!("{:?}", color3);
    color3.make_darker(0.78);
    println!("{:?}", color3);
}
