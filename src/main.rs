#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;

pub mod schema;
pub mod model;

use model::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


// Function to list all the groceries in our table
fn show_groceries() {
    let connection = establish_connection();
    let results = schema::groceries::table.load::<Groceries>(&connection).expect("Some Error occured");
    println!("Groceries in your inventory");
    println!("--------------------------------\n");

    if results.len() > 0 {
        for item in results {
            println!("Item ID: {}", item.id);
            println!("Type: {}", item.item_type);
            println!("Name: {}", item.item_name);
            println!("Quantity: {}", item.quantity);
            println!("Price: {}Rs", item.price);
            println!("--------------\n");
        }
    }
    else {
        println!("There are no items in your inventory");
    }
    println!("-------------------------------------");
}


// Function to insert new item in database
fn insert_grocery() {
    let mut name = String::new();
    let mut quantity = String::new();
    let mut price = String::new();
    let mut item_type = String::new();
    
    let connection = establish_connection();

    println!("Enter the name of the product: ");
    io::stdin().read_line(&mut name).unwrap();
    let name: String = name.to_lowercase().trim().parse().unwrap();

    println!("Enter the quantity of the product: ");
    io::stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim().parse().unwrap();

    println!("Enter the price of the product: ");
    io::stdin().read_line(&mut price).unwrap();
    let price: i32 = price.trim().parse().unwrap();

    println!("Enter the type of the product: ");
    io::stdin().read_line(&mut item_type).unwrap();
    let item_type: String = item_type.trim().parse().unwrap();

    let new_grocery = NewGrocery {
        item_name: name,
        quantity: quantity,
        price: price,
        item_type: item_type
    };

    diesel::insert_into(schema::groceries::table).values(&new_grocery).get_result::<Groceries>(&connection).expect("Could not insert new grocery");
    println!("New item added\n");
}


// Function to update item
fn update_grocery() {
    let connection = establish_connection();

    let mut id = String::new();
    println!("Enter the id of product that you want to update");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();

    let item = schema::groceries::table.find(id);

    let mut field = String::new();
    println!("Enter the name of field that you would like to change");
    io::stdin().read_line(&mut field).unwrap();
    let field: String = field.trim().parse().unwrap();

    let mut new_value = String::new();
    println!("Enter the new value");
    io::stdin().read_line(&mut new_value).unwrap();
    let new_value: String = new_value.trim().parse().unwrap();

    match field.as_ref() {
        "item_name" => match diesel::update(item).set(schema::groceries::item_name.eq(new_value)).get_result::<Groceries>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "item_type" => match diesel::update(item).set(schema::groceries::item_type.eq(new_value)).get_result::<Groceries>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "quantity" => match diesel::update(item).set(schema::groceries::quantity.eq(new_value.parse::<i32>().unwrap())).get_result::<Groceries>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        "price" => match diesel::update(item).set(schema::groceries::price.eq(new_value.parse::<i32>().unwrap())).get_result::<Groceries>(&connection) {
            Ok(some) => some,
            Err(_) => {println!("Please enter the correct id"); return}
        },
        _ => {println!("Could not recognize the field"); return}
    };

    println!("Update succesful")
}


// function to delete an item
fn delete_grocery() {
    let connection = establish_connection();

    let mut id = String::new();
    println!("Enter the id of product that you want to delete");
    io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().unwrap();

    diesel::delete(schema::groceries::table.find(id)).execute(&connection).expect("Could not delete the item");
    println!("One item deleted");
}


fn main() {
    println!("Welcome to your local grocery inventory\n");

    loop {
        println!("\nDo you want to:\n'Show'\n'Add'\n'Change'\n'Delete'\n'Exit'");
        println!("Type Any keyword to start the process: ");

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: String = command.to_lowercase().trim().parse().unwrap();

        match command.as_ref() {
            "show" => show_groceries(),
            "add" => insert_grocery(),
            "change" => update_grocery(),
            "delete" => delete_grocery(),
            "exit" => break,
            _ => println!("Not a correct key word")
        }
    }
    println!("Thank you for using the service\nBye Byee");
}