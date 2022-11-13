// Convert temperatures between Fahrenheit and Celsius.
fn temp_converter(deg: f32, to_unit: &mut char) -> Result<f32, String> {
    match to_unit.to_ascii_lowercase() {
        'f' => Ok(deg * 1.8 + 32.0),
        'c' => Ok((deg - 32.0) / 1.8),
        _ =>  Err("to_unit function arg must be: 'C','c','F','f'".to_string())
    } 
}

// Generate the nth Fibonacci number.
// used usize here to be able to leverage return of enumerate() in my test in main().
fn nth_fib_num(n: usize) -> i32 {
    if n < 2 { n as i32 } else { nth_fib_num(n - 1) + nth_fib_num(n - 2) }
}


// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
// no return type here since I just want to println!.................i think.
fn lyrics(day: i8) {

    if day > 0 && day < 13 {
        let suffix = if day == 1 {
            "st"
        } else if day == 2 {
            "nd"
        } else if day == 3 {
            "rd"
        } else {
            "th"
        };

        println!("On the {day}{suffix} day or Christmas, my true love gave to me");
        let rnge = (1..=day).rev();

        for d in rnge {
            match d {
                12 => println!("Twelve drummers drumming..."),
                11 => println!("Eleven pipers piping..."),
                10 => println!("Ten lords a-leaping..."),
                9 => println!("Nine ladies dancing..."),
                8 => println!("Eight maids a-milking..."),
                7 => println!("Seven swans a-swimming..."),
                6 => println!("Six geese a-laying..."),
                5 => println!("Five golden rings..."),
                4 => println!("Four calling birds..."),
                3 => println!("Three french hens..."),
                2 => println!("Two turtle doves..."),
                1 =>  if day > 1 {println!("and A partridge in a pear tree.")} else {println!("A partridge in a pear tree.")},
                _ => println!("only 12 days dude")
            };
        }
        lyrics(day - 1);
    }

}


// fn main() -> Result<(), String> {
fn main() -> Result<(), String> {
    // test temp_converter
    const F_BOILING: f32 = 212.0;
    const C_BOILING: f32 = 100.0;
    let tests = ['c', 'C', 'f', 'F'];
    // let tests_w_err = ['A', 'c', 'C', 'f', 'F'];

    for mut chr in tests {
        if chr == 'c' || chr == 'C' {
            assert_eq!(temp_converter(F_BOILING, &mut chr)?, C_BOILING)
        } else {
            assert_eq!(temp_converter(C_BOILING, &mut chr)?, F_BOILING)
        }
    }
    println!("* temp_converter..........PASSED");


    // test nth_fib_num w/for loop.
    let e = [0,1,1,2,3,5].iter().enumerate();
    for item in e {
        assert_eq!(nth_fib_num(item.0), *item.1);
    }


    // test nth_fib_num with .for_each()  ............felt closure cute, might delete later 😉
    [0,1,1,2,3,5].iter().enumerate().for_each(|(i, x)| assert_eq!(nth_fib_num(i), *x));
    println!("* nth_fib_num..........PASSED");


    // test xmas_lyrics
    lyrics(12);

    Ok(())
}