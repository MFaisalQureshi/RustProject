use std::io;
use std::f64;


fn main(){
  let mut key=String::new();
  println!("enter function you want to perform" );
  io::stdin().read_line( &mut key).ok();
  let key: i32=key.trim().parse().unwrap();
 
  if key== 1 {
    println!("you want to perform Addition", );
  add();
  }
  else if key== 2  {
    println!("you want to perform Subtraction", );
    sub();
  }
  else if key== 3  {
    println!("you want to perform Multiplication", );
    mul();
  }
 else if key== 4 {
    println!("you want to perform Division", );
    div();
  }
  else if key== 5 {
    println!("you want to perform Square", );
    square();
  }
  else if key== 6  {
    println!("you want to perform Cube", );
    
    cube();
  }
  else if key== 7  {
    println!("you want to perform Square Root", );
    
    cube();
  }
  else if key== 8  {
    println!("you want to Know Square Root", );
     squareroot();
  }
  else if key== 9  {
    println!("you want to Know Remainder", );
    modules();
  }
  else if key== 10  {
    println!("you want to Know Cube root ", );
    cuberoot();
  }
   else if key== 11  {
    println!("you want to Know log value", );
    log();
  }
  else if key== 12  {
    println!("you want to Know Sin value", );
    sinfun();
  }
  else if key== 13  {
    println!("you want to Know Cos value", );
    cosfun();
  }
   else if key== 14  {
    println!("you want to Know Tan value", );
    tanfun();
  }
   else if key== 15  {
    println!("you want to Know Factorial", );
    factorial();
  }
   

  else {
    println!("You enter wrong key" );
  }
  

}
fn add() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1+no2) );
 
  
  
  
}
fn sub() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1-no2) );
 
  
}
fn mul() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: i32=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: i32=no2.trim().parse().unwrap();

  println!("Result is {}",(no1*no2) );
 
  
}
fn div() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let  no2: f64=no2.trim().parse().unwrap();

  println!("Result is {}",(no1/no2) );
 
  
}
fn square() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",no1.powi(2) );
 
  
}
fn cube() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",(no1*no1*no1) );
 
  
}
fn squareroot() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
  

  println!("Result is {}",no1.sqrt());
 
  
}
fn modules() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 let mut no2=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no2).ok();
  let no2: f64=no2.trim().parse().unwrap();
 
  

  println!("Result is {}",no1%no2);

 
  
}
fn cuberoot() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.cbrt());

 
  
}
fn log() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.log10());

 
  
}
fn sinfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f32=no1.trim().parse().unwrap();
  
 
  

  println!("Result is {}",no1.sin());

 
  
}
fn cosfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f32=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.cos());

 
  
}
fn tanfun() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let no1: f64=no1.trim().parse().unwrap();
 
 
  

  println!("Result is {}",no1.tan());

 
  
}
fn factorial() {
  let mut no1=String::new();
  println!("enter  number" );
  io::stdin().read_line(&mut no1).ok();
  let mut no1: i32=no1.trim().parse().unwrap();
  let mut fact=1;
while no1>0 {
    fact=fact*no1;
    no1=no1-1;
}
println!("Factorial of the number is {} ",fact);
}
 
 
  

  

 
  






