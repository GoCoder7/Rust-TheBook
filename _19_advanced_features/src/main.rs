
fn main() {
    _1_raw_pointer_dereference();
    _2_unsafe_function();
    _3_extern();
    _4_mutable_static_variable();
    _5_impl_add();
    _6_meters_to_millimeters();
    _7_calling_methods_in_same_name();
    _8_calling_function_in_same_name();
    _9_super_trait();
    _10_newtype_extension();
    _11_type_alias();
    _12_never_type();
    _13_function_pointer();
    _14_function_pointer_for_generator();
    _15_return_closure();
    _16_declarative_macro();
    _17_procedural_macro();
}

use core::slice;


fn _1_raw_pointer_dereference() {
    let mut num = 5;
    let r1 = &num as *const i32; // rawpointer로 캐스팅(불변)
    let r2 = &mut num as *mut i32; // rawpointer로 캐스팅(가변)
    unsafe { // 
        println!("r1 : {}", *r1);
        *r2 += 1;
        println!("r1 : {}", *r1);
        println!("r2 : {}", *r2);
    }
}

fn _2_unsafe_function(){
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid), // unsafe 환경에서만 사용할 수 잇음
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn _3_extern() {
    extern "C" {
        fn abs(input: i32) -> i32; // C언어의 abs함수 가져옴
    }
    unsafe {
        println!("abs -3 : {}", abs(-3))
    }
}

fn _4_mutable_static_variable() {
    static mut COUNTER : u32 = 0; // 가변정적변수는 unsafe환경에서만 접근할 수 있음
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn _5_impl_add() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Self; // 연관타입을 지정해야 함

        fn add(self, rhs: Self) -> Self::Output { 
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

fn _6_meters_to_millimeters() {
    use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters { // +연산 우측타입(Rhs)을 Meters로 지정
        type Output = Self; 
    
        fn add(self, rhs: Meters) -> Self::Output {
            Self (self.0 + rhs.0 *1000)
        }
    }
}

fn _7_calling_methods_in_same_name() {
    trait Pilot { fn fly (&self); }
    trait Wizard { fn fly (&self); }
    struct Human;
    impl Human {
        fn fly (&self) {println!("fly of human")}
    }
    impl Pilot for Human {
        fn fly (&self) {println!("fly of pilot")}
    } 
    impl Wizard for Human {
        fn fly (&self) {println!("fly of wizard")}
    } 
    let human = Human;
    human.fly(); // Human의 fly메서드 호출(직접구현한 fly메서드가 실행됨)
    Pilot::fly(&human); // Human에 구현한 Pilot의 fly를 실행(human을 통해 Human타입을 유추)
    Wizard::fly(&human); // Human에 구현한 Wizard의 fly를 실행(human을 통해 Human타입을 유추)
    Human::fly(&human); // Human에 직접구현한 fly를 실행
    <Human as Wizard>::fly(&human); // Human에 구현한 Wizard의 fly를 실행(직접 명시)
}

fn _8_calling_function_in_same_name() {
    trait Animal { fn baby_name() -> String; }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            "Spot".to_owned()
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            "puppy".to_owned()
        }
    }
    println!("dog name : {}", <Dog as Animal>::baby_name()); //메서드가 아니므로 타입유추가 불가하여, Dog에 구현한 것임을 명시해야 함
}

fn _9_super_trait() {
    use std::fmt::Display;
    trait OutlinePrint: Display {
        fn ouline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat((len + 2)));
            println!("* {} *", output);
            println!("*{}*", " ".repeat((len + 2)));
            println!("{}", "*".repeat(len + 4));
        }
    }
    
    struct Point {
        x: i32,
        y: i32,
    }
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    let pt = Point{x:5000, y:10000};
    pt.ouline_print();
}

fn _10_newtype_extension() {
    use std::fmt;
    // 현재crate에 속하지 않은 String struct에, 현재crate에 속하지 않은 Display trait을 구현할 수 없으므로
    // 별도의 래퍼(요소1개 튜플형 struct)로 래핑하여, Display trait을 구현
    struct Wrapper(Vec<String>); 
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let s = Wrapper(vec!["hi".to_owned(), "hello".to_owned()]);
    println!("{}", s)
}

fn _11_type_alias() {
    type Result<T> = std::result::Result<T, std::io::Error>;
    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()>;
    }
}

fn _12_never_type() {
    let opt = Some(5); // i32
    // 1. panic!
    let some = match opt { // i32
        Some(s) => s,
        None => panic!("값 없음"),
    };
    // 2. continue
    for _ in 0..10 {
        let some = match opt { // i32
            Some(s) => s,
            None => continue,
        };
    }
    // 3. loop(without break)
    let some = match opt { // i32
	Some(s) => s,
	None => loop{},
    };

    fn is_equal<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
        t1 == t2
    }
    is_equal("Hello", "World");
}

fn _13_function_pointer() {
    type ReturnSame<T> = fn(T) -> T;
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice<T>(f: ReturnSame<T>, arg: T) -> T {
        f(f(arg))
    }

    let answer = do_twice(add_one, 5);
    println!("{answer}");
}

fn _14_function_pointer_for_generator() {
    struct Count(u32);
    let list_of_counts: Vec<Count> = (0..20).map(Count).collect();
    let list_of_statuses: Vec<Option<u32>> = (0..20).map(Option::Some).collect();
}

fn _15_return_closure() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}

fn _16_declarative_macro() {
    #[macro_export]
    macro_rules! vec_1 {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let vec = vec_1![1,2,3];
    println!("{vec:?}");
}

mod hello_macro;
fn _17_procedural_macro() {
    use hello_macro::HelloMacro;

    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }

    Pancakes::hello_macro();
}