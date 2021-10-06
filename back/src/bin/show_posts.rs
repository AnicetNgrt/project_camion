extern crate camion;
extern crate diesel;

use self::camion::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    let connection = establish_connection();
    let results = schema::posts::table.filter(schema::posts::published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}