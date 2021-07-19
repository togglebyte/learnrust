fn return_something(maybe_true: bool) -> usize {
    if maybe_true {
        123
    } else if 1 == 1 {
        111
    } else {
        0
    }
}

fn main() {
    // || = or
    // && = and

    let val = false;

    let range = 0..10;

    // For loop
    for mut i in range {
        eprintln!("{}", i);
        if i >= 10 {
            break
        }
    }

    let mut val = 0;

    // While loop
    while val < 10 {
        eprintln!("{}", val);
        val += 1;
        if val >= 3 {
            break
        }
    }

    let mut val = 0;

    // Assignment via loop
    let final_val = loop {
        eprintln!("{}", val);
        val += 1;
        if val == 10 {
            break val;
        }
    };

    let mut x = 0;
    let mut y = 0;
    'x_value: loop {
        x += 1;
        'y_value: loop {
            y += 1;
            if y > 10 {
                break 'x_value;
            }
        }
    }

    eprintln!("final: {:?}", final_val);

}
