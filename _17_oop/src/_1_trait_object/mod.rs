pub fn main() {
    allow_variant_types_with_trait_object();
}
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}
impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
    }
}

fn allow_variant_types_with_trait_object(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_owned(),
                    "No".to_owned(),
                    "Maybe".to_owned(),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_owned(),
            })
        ]
    };
    screen.run();
}