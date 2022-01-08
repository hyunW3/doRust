use std::io;

fn main() {
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        let y = another_function(x,'h');
        println!("The value of x is : {} and {}",x,y);
    }
    
    println!("The value of x is : {}",x);

    let a = [1,2,3,4,5];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse()
                            .expect("Index entered was not a number" );
    let element = a[index];
    println!(
        "The value of the element at index {} is : {}", index,element
    );
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is add after loop {}",result);
    while counter != 0 {
        println!("counter value : {}",counter);
        counter -=2;
    }
    for element in a {
        println!("the value is {}",element);
    }
    for number in (1..4).rev() {
        println!("{}!",number);
    }
    println!("LiftOff!");
    
    let mut s = String::from("hello world");
    change(&mut s);
    println!("s is : {}",s);

    let word = first_word(&s);
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("word is : {}",word);
    println!("{} {}",hello,world);
    s.clear();
}
fn change(some_string: &mut String){
    some_string.push_str(", world");
}
fn another_function(x: i32, unit_label: char) -> i32 {
    println!("Another function. {}{}",x,unit_label);
    x+1
}
fn first_word(s : &String) -> &usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
