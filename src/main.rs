fn main() {
    let i = 10; // we cannot modify this value.
    let mut ii =20;// we can modify this value.
    const C:i8= 30; // const stay alive  till the program is alive.
    println!("Hello, world!");
    let hex = 0xff;
    let oct =0o377;
    let bin=0b1111_1111;
    let dec=02_55;
    println!("{} {} {} {}",hex,oct, bin ,dec);
//      rust had 4 scalar data types
//     int char float and boolean
// int can be i8,i16 , i 32 , i64 , i128
//     int can be signed can unsigned
    let inta:u8= 35;
    println!("{}",inta);
 let byte=b'A';
    println!("{}",byte);
    let _y=10; // here _ will silence the warning that we didnt use the variable y after declaring
    let x = 2.0;
    let xx:f32 =3.0;
    let t=true;
    let f:bool =false;
     let other_c = 'c';
    let a =10;
    let b= 30;
    let rem= a %b;
    println!("{}",rem);
// premitive compound type
//     we have tuple and array
//     tuple cannot be edited but could contain different datatype elements in it.
    let tup = (10,"hi",345);
    println!("{}",tup.0); // accessing content of a tuple
        let (aaa,bbb,ccc)=tup;
//    array will have length but will have same type in it by default , if we make it mutable
// we will be able to modify the value of the array.
    let arr=[1,3,4];
    println!("{}",arr[2]);
//         vectors are resizable instances of variables that reside in heap memory.
//     by using vec macro
    let mut nums = vec![1,2,3];
    nums.push(4     as i32);
    println!("{:?}",nums);// this will print all the numbers
    nums.pop();
    println!("{:?}",nums);
//     another method
    let mut nums1 = Vec::new();
    // this is essentially calling the vec![] like in line 41 , vector elements must be of
    // same type, We can add values to a Vector by creating a mutable
    nums1.push(1);
    nums1.push(11);
    println!("{:?}",nums1);
    nums1.reverse();

    nums1.len();
    println!("{:?}",nums1);
    let mut numm = Vec::<i32>::with_capacity(4);//
//     creating the vector with capacity during initialization.
    let v:Vec<i32>=(0..5).collect();
    println!("{:?}",v);
    let sv:&[i32] = &v;
    println!("{:?}",sv);
    let sv1:&[i32] = &sv[2..4];
    println!("{:?}",sv1);
//     sv and sv1 is directly pointing to data in v and its non owning reference
//     ordinary reference is non owning point of reference to a single value
//     but with slice its a reference to a range of continuous values.
    
// slice is most useful when a function operates on a array or a vector we use slice
// strings are very similar to vectors as they are stored as a vector of bytes.
//      string is always UTF8 and its heap allocated and not null terminated and
//
    let name =String::from("NG");
    let like = "Pizza".to_string();
    let name_new=name.replace("NG","naga ganesh");
println!("{}",name);
println!("{}",name_new);
//     string slice
//     its a fat pointer containing address and data , a string slice doesnt
//     allocate memory in the heap.
    let stri ="hello";
    print!("{}",stri);
// print!("{}",stri.bogus());

let sstere =stri.to_string();
    let sttt=&sstere;
    println!("{}",sttt);
    println!("{}",sstere);
// == string are equal , != strings are not equal
    println!("{}","ONe" =="one");
    println!("{}","ONe".to_lowercase() =="one");

// string literal
//  some time we dont want valid utf-8 characters thats when we use string literals

    let sst = "\x52\x75\x73\x74";
    println!("{}",sst);
    print_phase("this is a phrase");
    println!("{}",gcd(10,20));
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