

//*basic data types*/  

// Boolean 
// let is_active: bool = true;

// Integer 
// let age: u8 = 30


// Float 
// let pi: f64 = 3.14

// Double 
// let e: f64 = 2.718281828459045

// Character 
// let c: char = 'c';

// String 
// let name: String = "Moncef"


// Arrays 
// let numbers: [i32; 5] = [1,2,3,4,5];

// Tuple 
// let tup: (i32, f64, u8) = (500, 6.4, 1);


// Struct 
// struct Person {
//     name: String,
//     age: u8
// }


//*variables */

//? by default variables are not mutable (you cant change them) , you can change that by passing (mut) after let and let them mutable

// let two  = 2;
// let hello = "Hello";
// let j = 'j';
// let my_half = 0.5;
// let my_bool = true;
// let mut my_name = "Moncef";
// let your_half = my_half;


//*Functions */
// fn name(a: i32, b: i32) -> i32 {
//      a + b //function body
//  }

// let x = name(1,1);
// let y = name(3,0);
// let z = name(x,1);





//*The println macro */
// println => print to the console
// println! => print to the console with a format



// "{life:?}" => for debugging
// "{life}" => println to the console (the user can see it )
// fn main() {
// let life  = 100;
// println!("Hello");
// println!("{:?}", life);
// println!("{:?} {:?}", life, life);
// println!("the meaning is {:?}", life);
// println!("{life:?}");// for debug
// println!("{life}");//fro printing in the console 
// }




//* control flow "if" */
//? if condition 
// let life = 1000;
// if life > 100 {
//     println!("big number");
// } else {
//     println!("small number");
// }//* big number

//? nested if/else condition 
// let a = 150;
//  if a >100{

//  if a >200{
//     println!("Huge number");
//  }else {
//      println!("big number");
//  }
//  }else{
//     println!("small number");
//  }//* big number


//? if else if condition 
// let a = 99;
// if a > 200 {
//     println!("huge number");
// } else if a > 99 {
//     println!("big number");
// } else {
//     println!("small number");
// }


//! this will not work
// if a > 99 {
//     println!("big number");
// }else if a>200 {
//     println("huge number");
// }else {
//     println("small number");
// }



//* Repetition using loops */

//*using loop */

// let mut a = 1 ; //? mut for making the variable changeable
// loop {
//     if a == 5 {
//     println!("the number is {:?}", a);
//         break;
//     }
//     println!("the number is less or more then {:?}", a);
//     a += 1;
// }=>//* in the end the a will be 5   */

//*using while loop */
// let mut a = 1 ; 
// while a != 5 {
//     println!("the number is less or more then {:?}",a);
//     a += 1;
// }=> //* in the end the a will be 5   */

  




fn main(){
}









// fn main() {

// }










