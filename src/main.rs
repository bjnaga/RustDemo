


fn main() {
    let i = 10; // we cannot modify this value.
    let mut ii = 20;// we can modify this value.
    const C: i8 = 30; // const stay alive  till the program is alive.
    println!("Hello, world!");
    let hex = 0xff;
    let oct = 0o377;
    let bin = 0b1111_1111;
    let dec = 02_55;
    println!("{} {} {} {}", hex, oct, bin, dec);
//      rust had 4 scalar data types
//     int char float and boolean
// int can be i8,i16 , i 32 , i64 , i128
//     int can be signed can unsigned
    let inta: u8 = 35;
    println!("{}", inta);
    let byte = b'A';
    println!("{}", byte);
    let _y = 10; // here _ will silence the warning that we didnt use the variable y after declaring
    let x = 2.0;
    let xx: f32 = 3.0;
    let t = true;
    let f: bool = false;
    let other_c = 'c';
    let a = 10;
    let b = 30;
    let rem = a % b;
    println!("{}", rem);
// premitive compound type
//     we have tuple and array
//     tuple cannot be edited but could contain different datatype elements in it.
    let tup = (10, "hi", 345);
    println!("{}", tup.0); // accessing content of a tuple
    let (aaa, bbb, ccc) = tup;
//    array will have length but will have same type in it by default , if we make it mutable
// we will be able to modify the value of the array.
    let arr = [1, 3, 4];
    println!("{}", arr[2]);
//         vectors are resizable instances of variables that reside in heap memory.
//     by using vec macro
    let mut nums = vec![1, 2, 3];
    nums.push(4 as i32);
    println!("{:?}", nums);// this will print all the numbers
    nums.pop();
    println!("{:?}", nums);
//     another method
    let mut nums1 = Vec::new();
    // this is essentially calling the vec![] like in line 41 , vector elements must be of
    // same type, We can add values to a Vector by creating a mutable
    nums1.push(1);
    nums1.push(11);
    println!("{:?}", nums1);
    nums1.reverse();

    nums1.len();
    println!("{:?}", nums1);
    let mut numm = Vec::<i32>::with_capacity(4);//
//     creating the vector with capacity during initialization.
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);
    let sv: &[i32] = &v;
    println!("{:?}", sv);
    let sv1: &[i32] = &sv[2..4];
    println!("{:?}", sv1);
//     sv and sv1 is directly pointing to data in v and its non owning reference
//     ordinary reference is non owning point of reference to a single value
//     but with slice its a reference to a range of continuous values.

// slice is most useful when a function operates on a array or a vector we use slice
// strings are very similar to vectors as they are stored as a vector of bytes.
//      string is always UTF8 and its heap allocated and not null terminated and
//
    let name = String::from("NG");
    let like = "Pizza".to_string();
    let name_new = name.replace("NG", "naga ganesh");
    println!("{}", name);
    println!("{}", name_new);
//     string slice
//     its a fat pointer containing address and data , a string slice doesnt
//     allocate memory in the heap.
    let stri = "hello";
    print!("{}", stri);
// print!("{}",stri.bogus());

    let sstere = stri.to_string();
    let sttt = &sstere;
    println!("{}", sttt);
    println!("{}", sstere);
// == string are equal , != strings are not equal
    println!("{}", "ONe" == "one");
    println!("{}", "ONe".to_lowercase() == "one");

// string literal
//  some time we dont want valid utf-8 characters thats when we use string literals

    let sst = "\x52\x75\x73\x74";
    println!("{}", sst);
    print_phase("this is a phrase");
    println!("{}", gcd(10, 20));
    println!("{} ", multi_return_value(false));
    if 1 > 0 {
        println!("{}", true);
    }else if 1==1 {
        println!("{}", true);
    }
    else {
        println!("{}", false);
    }
// infinite loop
    // loop{
    //     println!("loop", );
    // }

//     loop with a name
let mut num =0;
    // 'count:loop{
    //     println!("Count: {}",1);
    //     let mut decrease =5;
    //     loop{
    //         println!("Decreasing {}", decrease);
    //         if decrease == 4 {
    //             break;
    //         }
    //         if num==2{
    //             decrease -= 1;
    //         }
    //         num += 1;
    //     }
    // }
    while num < 5{
        println!("Run: {}", num);
        num +=1;
    }
//     for loop is best suitable to work on collection
    let vec :Vec<i8>=(0..10).collect();
    for element in vec{
        println!("{}", element);
    }
    // prints reverse of number 1 to 6 not including 6
    for numm in (1..6).rev(){
        println!("{}", numm);
    }


    // ownership module to manage memory safety errors
//  such as dangling pointers, double freeze using uninitialized memory safety and so on
// this are the advantages of this mechanism
// stack(LIFO) and heap- its unorganized, we request the space and address is returned
// pushing data on stack and searching data on stack is always faster than data on heap
// each variable in rust is associated with owner and therefore there can be only one owner
// when owner goes out of scope memory is freed

        let var =1; // created on stack
        let mut s ="Hello world".to_string(); // created on heap
        s.push_str(",world");
    println!("{}", s);

//     move - movie the ownership of value from one type to another

    let x=vec!["naga".to_string()];
    let y=x; // here x has moved the ownership to y so if we print we will get error
    println!("{:?}", y);
// copy - deep and shallow copy
// clone - if we want to take a variable value without taking ownership we use clone instead
//     it performs a deep copy
//     it may be expensive
    let xx=vec!["naga".to_string()];
    let yy =xx.clone();
    let zz = yy.clone();
    println!("{:?}", zz);
    println!("{:?}", yy);
    println!("{:?}", xx);
// copy
    let xxx=1;
    let yyy=xxx;
    println!("x ={}, y ={}", xxx, yyy); // this works because most types implement a move
//     some implement copy , copy is implemented by type stored in stack , like int, float , char,
//     bool, but tuple can have copy traits if every value it contains implements copy


//     how moves work?
     let s= String::from("hello world");
    takes_ownership(s);// gives ownership to fn take_ownership
    // println!("{}", s);

    let ii =1;
    takes_copy(ii);

    let str1= give_ownership();
    println!("{}", str1);

    let str2 = take_ownership_give_ownership("naga".to_string());
    println!("{}", str2);


    // references and borrowing - allow us to make references to a value without borrowing ownership
//     2 type of references
//     shared reference vs mutable reference
//     shared reference allows us to read, we can have n number of references to value
//     mutable reference we can read and write , so we can have only one reading and writing it


    let mut strr=String::from("naga");
    exchange_str(& mut strr);  // we are sending and retrieving the string ownership
    println!("{}",strr);

}
fn exchange_str(strr:&mut String ){
    strr.push_str("ganesh")
}
// ownership example function
fn takes_ownership(some_string: String) {
    let str = some_string;
    println!("{}", str);
}
// ownership copy
fn takes_copy(str:i32) {
    let str = str;
    println!("{}", str);
}
fn give_ownership()-> String{
    "ownership given".to_string()
}

fn take_ownership_give_ownership(ownership: String)-> String{
    ownership
}


//  function have to be named using snake_case
fn print_phase(phrase: &str) {
    println!("Inside the function :{}",phrase);

}
fn gcd(mut a:i32 , mut b:i32)->i32   {
    while a!=0{
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a= a%b;
    }
    // return a value by not including ; at the end.
    b
}

fn multi_return_value(flag:bool) -> bool{
    if flag{
         true
    }
    else{
            false
    //     simple false will work fine as well
    }
}