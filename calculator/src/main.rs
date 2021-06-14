use std::io;
fn main() {
    let mut choice=String::new();
	println!("............................................................................................");
	println!("1.Addition\n2.Subtraction\n3.Multiplication\n4.Division\n5.Modulo\n6.Square\n7.Cube");
	println!("............................................................................................");
	println!("\nWhat you want to perform:");
	io::stdin().read_line(&mut choice).ok();
	let choice: i32=choice.trim().parse().unwrap();
	
	if choice==1
	{
		add();
	}
	else if choice==2
	{
		sub();
	}
	else if choice==3
	{
		mul();
	}
	else if choice==4
	{
		div();
	}
	else if choice==5
	{
		modulo();
	}
	else if choice==6
	{
		square();
	}
	else if choice==7
	{
		cube();
	}
	else
	{
		println!("Choice is wrong!!!");
	}
	
}
fn add()
{
	let mut num1=String::new();
	println!("Enter two numbers:");
	io::stdin().read_line(&mut num1).ok();
	let num1: i32=num1.trim().parse().unwrap();
	
	let mut num2=String::new();
	io::stdin().read_line(&mut num2).ok();
	let num2: i32=num2.trim().parse().unwrap();
	
	println!("Addition: {}",(num1+num2));
}
fn sub()
{
	let mut num1=String::new();
	println!("Enter two numbers:");
	io::stdin().read_line(&mut num1).ok();
	let num1: i32=num1.trim().parse().unwrap();
	
	let mut num2=String::new();
	io::stdin().read_line(&mut num2).ok();
	let num2: i32=num2.trim().parse().unwrap();
	
	println!("Subtraction: {}",(num1-num2));
}
fn mul()
{
	let mut num1=String::new();
	println!("Enter two number:");
	io::stdin().read_line(&mut num1).ok();
	let num1: i32=num1.trim().parse().unwrap();
	
	let mut num2=String::new();
	io::stdin().read_line(&mut num2).ok();
	let num2: i32=num2.trim().parse().unwrap();
	
	println!("Multiplication: {}",(num1*num2));
}
fn div()
{
	let mut num1=String::new();
	println!("Enter two number:");
	io::stdin().read_line(&mut num1).ok();
	let num1: i32=num1.trim().parse().unwrap();
	
	let mut num2=String::new();
	io::stdin().read_line(&mut num2).ok();
	let num2: i32=num2.trim().parse().unwrap();
	
	println!("Division: {}",(num1/num2));
}
fn modulo()
{
	let mut num1=String::new();
	println!("Enter two number:");
	io::stdin().read_line(&mut num1).ok();
	let num1: i32=num1.trim().parse().unwrap();
	
	let mut num2=String::new();
	io::stdin().read_line(&mut num2).ok();
	let num2: i32=num2.trim().parse().unwrap();
	
	println!("Modulo: {}",(num1%num2));
}
fn square()
{
	let mut no=String::new();
	println!("Enter two number:");
	io::stdin().read_line(&mut no).ok();
	let no: i32=no.trim().parse().unwrap();
	
	println!("Square: {}",no*no);
}
fn cube()
{
	let mut no=String::new();
	println!("Enter two number:");
	io::stdin().read_line(&mut no).ok();
	let no: i32=no.trim().parse().unwrap();
	
	println!("Cube: {}",no*no*no);
}