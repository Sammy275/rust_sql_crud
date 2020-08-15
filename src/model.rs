use crate::schema::groceries;
// Models
#[derive(Queryable)]
pub struct Groceries {
    pub id: i32,
    pub item_name: String,
    pub quantity: i32,
    pub price: i32,
    pub item_type: String
}

#[derive(Insertable)]
#[table_name="groceries"]
pub struct NewGrocery {
    pub item_name: String,
    pub quantity: i32,
    pub price: i32,
    pub item_type: String
}