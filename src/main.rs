fn main() {
    println!("{}", "Call To helloworld()") ;
    hello_world();

    println!("\n{}", "Call To fibo") ;
    println!("{}", fib(6));

    println!("\n{}", "Call To fibo recursive") ;
    println!("{}", fib_recur(1,1,1,6));

    let mut global_fir: i32 = 1;
    let mut global_sec: i32 = 1;
    let mut global_ret: i32 =1;

    println!("\n{}", "Call To fibo borrow") ;
    fib_borrow(&mut global_fir, &mut global_sec, &mut global_ret, 6);
    println!("{}", global_ret );

    global_fir = 1;
    global_sec = 1;
    global_ret = 1;
    println!("\n{}", "Call To fibo borrow recur") ;
    fib_borrow_recur(&mut global_fir, &mut global_sec, &mut global_ret, 6);
    println!("{}", global_ret );
}

fn hello_world(){
    println!("Hello, world!");
}


fn fib(x: i32) -> i32 {

    let mut fir = 1;
    let mut sec = 1;
    let mut ret = 1;

    let mut index = x.clone();
    if x <= 2 {
        ret = 1;
    } else {
        while index > 2 {

            ret = fir + sec;
            fir = sec;
            sec = ret;
            index -= 1;

        }

    }

    return ret

}

fn fib_recur(mut fir: i32, mut sec: i32, mut ret: i32, index: i32) -> i32 {
    if  index <= 2{
        ret
    }else{

        ret = fir + sec;
        fir = sec;
        sec = ret;

        fib_recur(fir, sec, ret, index -1)
    }
}

fn fib_borrow(fir: &mut i32, sec: &mut i32, ret: &mut i32, mut index:  i32){

    while index > 2{

        *ret = *fir + *sec;
        *fir = *sec;
        *sec = *ret;

        index -= 1;

        }

}

fn fib_borrow_recur(fir: &mut i32, sec: &mut i32, ret: &mut i32, index:  i32){

    if index > 2{
        *ret = *fir + *sec;
        *fir = *sec;
        *sec = *ret;
        fib_borrow_recur(fir, sec, ret, index - 1);
    }
}
