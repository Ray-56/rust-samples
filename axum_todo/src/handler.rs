pub async fn usage() -> &'static str {
    r#"
        ### USAGE ###
        - GET /todo -- get all todo list
        - POST /todo -- create a todo list
        - GET /todo/:list_id -- get detail for a todo list
        - DELETE /todo/:list_id -- delete a todo list, include it's items
        - PUT /todo/:list_id -- edit a todo list
        - GET /todo/:list_id/items -- get items from todo list
        - GET /todo/:list_id/items/:item_id -- get detail for a todo item
        - PUT /todo/:list_id/items/:item_id -- edit a todo item(set the item to checked)
        - DELETE /todo/:list_id/items/:item_id -- delete a todo item
    "#
}