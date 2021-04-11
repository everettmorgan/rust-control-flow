fn main () {
    if_else ();
    if_as_expr ();
    loop_1 ();
    loop_2 ();
    loop_3 ();
    loop_4 ();
    loop_5 ();
    fibonacci (10);
    far_to_cel (100.0);
}

fn far_to_cel (f : f64) {
    let conv = (f - 32.0) * (5.0 / 9.0);
    println! ("{}F is equivalent to {:.3}C", f, conv);
}

fn fibonacci (x : i32) {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut next = 1;

    for _number in 1..x+1 {
        println! ("{}", next);
        next = n1 + n2;
        n1 = n2;
        n2 = next;
    }
}

fn loop_5 () {
    for number in (1..4).rev() {
        println! ("{}!", number);
    }
    println! ("LIFTOFF");
}

fn loop_4 () {
    let a : [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println! ("the value is: {}!", element);
    }
}

// expensive loop because we're checking the index value every loop
fn loop_3 () {
    let a : [i32; 5] = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < a.len() {
        println! ("the value is: {}!", a[index]);
        index += 1;
    }
}

fn loop_2 () {
    let mut counter = 3;

    while counter != 0 {
        println! ("{}", counter);
        counter -= 1;
    }

    println! ("LIFTOFF!");
}

fn loop_1 () {
    let mut counter = 1;
    let x : i32 = loop {
        counter += 1;
        if counter == 30 {
            break counter * 2
        }
    };
    println! ("x is: {}", x );
}

fn if_as_expr () {
    let condition = true;
    let number = if condition { 5 } else { 3 };
    println! ("number: {}", number);
}

fn if_else () {
    let x = 5;

    if x < 10 {
        println! ("less than 10");
    } else {
        println!( "greater than 10");
    }
}