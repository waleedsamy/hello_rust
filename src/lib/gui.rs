pub trait Draw {
    fn draw(&self) {
        println!("~~~~~~~~~~ draw...");
    }
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|x| x.draw())
    }
}

#[derive(Debug)]
struct Button {
    text: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("~~~~~~~~~~ draw button {:?}", self);
    }
}

#[derive(Debug)]
struct TextBox {
    text: String,
}
impl Draw for TextBox {
    fn draw(&self) {
        println!("~~~~~~~~~~ draw {:?}", self);
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw() {
        let v: Vec<Box<dyn Draw>> = vec![
            Box::new(Button {
                text: String::from("b1"),
            }),
            Box::new(TextBox {
                text: String::from("b1"),
            }),
        ];

        let screen = Screen { components: v };
        screen.run();
    }
}
