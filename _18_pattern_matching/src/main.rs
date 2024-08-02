
fn main() {
    _match();
    _if_let_else();
    _while_let();
    _for_in();
    _destructed_parameter();
    _refutable_but_needed_irrefutable();
    _enum_destruct();
    _extra_condition();
    _at_binding();
}



fn _match() {
    let x = Some(5);
    match x {
        Some(i) => println!("{i}"),
        None => (),
    }
}


fn _if_let_else() {
    let my_color: Option<&str> = None;
    let is_tuesday = false;
    let my_age: Result<u8, _> = "34".parse();

    if let Some(color) = my_color {
        println!("my Color = {color}");
    } else if is_tuesday {
        println!("I don't have any color but It's tuesday");
    } else if let Ok(age) = my_age {
        if age > 30 {
            println!("No color, and not tuesday, and older than 30");
        } else {
            println!("No color, and, not tuesday, and younger than or equal to 30 ")
        }
    } else {
        println!("no color, not tuesday, no age")
    }
}

fn _while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn _for_in() {
    let v = vec!['a', 'b', 'c'];

    for (i, v) in v.iter().enumerate() {
        println!("{v} is at index{i}");
    }
}

fn _destructed_parameter() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("x={x}, y={y}");
    }
    let point = (5,10);
    print_coordinates(&point);
}

fn _refutable_but_needed_irrefutable() {
    let my_option = Some(5);
    // let Some(x) = my_option; // let은 irrefutable만 받음
    if let x = 5 {
        println!("{x}");
    }
    enum MyNum {
        Num(i32),
    }
    let y = MyNum::Num(5);
    let MyNum::Num(z) = y;
}

fn _enum_destruct() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Move { x: 5, y: 10 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("msg:{text}"),
        Message::ChangeColor(0, g, b) => println!("r:0, g:{g}, b:{b}"),
        Message::ChangeColor(r, g, b)         => println!("r:{r}, g:{g}, b:{b}"),
    }
}

fn _extra_condition() {
    let num = Some(3);
match num {
	Some(n) if n % 2 == 0 => println!("even number"),
	Some(_) => println!("odd number"),
	None => (),
}

}

fn _at_binding() {
    let num = 4;
    match num {
        1 | 2 => println!("1 or 2"),
        n @ 3..=5  => println!("{n}"),
        _ => println!("not in 1~5"),
    }
}

