fn main() {
    
    // init params
    let low = 1;
    let high = 101;
    let fizz = 3;
    let buzz = 5;
    
    // init fizz_buzz program
    fn fizz_buzz(low: i32,
                 high: i32,
                 fizz: i32,
                 buzz: i32) {
    
        let fizzbuzz = fizz * buzz;

        for i in low..high {
            if i % fizzbuzz == 0 {
                assert_eq!(i % 15, 0);
                println!("FizzBuzz");
            } else if i % fizz == 0 {
                assert_eq!(i % 3, 0);
                println!("Fizz");
            } else if i % buzz == 0 {
                assert_eq!(i % 5, 0);
                println!("Buzz");
            } else {
                println!("{}", i);
            }
        }        
    }
    
    // run fizz_buzz program
    fizz_buzz(low, high, fizz, buzz);
    
}
