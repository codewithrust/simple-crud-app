// derive(Clone) is used to clone the struct object
#[derive(Clone)]
struct User {
    id: String,
    username: String,
    email: String,
}

fn main() {
    // init users vector
    // vector is used because we can add, update, delete items
    let mut users: Vec<User> = Vec::new();

    // init some hard coded users
    init_users(&mut users);

    // get all users
    get_users(&users);

    // add new user
    let user = User {
        id: String::from("3"),
        username: String::from("user3"),
        email: String::from("user3@example.com"),
    };
    add_user(&mut users, user);

    // update user
    let updated_user = User {
        id: String::from("3"),
        username: String::from("user3_updated"),
        email: String::from("user3_updated@example.com"),
    };
    update_user(&mut users, updated_user);

    let find_user_result = get_user_by_id(&users, "2");
    if find_user_result.is_err() {
        println!("\nGet User By ID: {}", find_user_result.err().unwrap());
    } else {
        let user = find_user_result.unwrap();
        println!("\nGet User By ID:");
        println!(
            "id: {}, username: {}, email: {}",
            user.id, user.username, user.email
        );
    }

    delete_user_by_id(&mut users, "3").unwrap();
}

fn init_users(users: &mut Vec<User>) {
    let user = User {
        id: String::from("1"),
        username: String::from("user1"),
        email: String::from("user1@example.com"),
    };

    let user_2 = User {
        id: String::from("2"),
        username: String::from("user2"),
        email: String::from("user2@example.com"),
    };

    // push users to users vector
    users.push(user);
    users.push(user_2);

    println!("Initial Users:");
}

fn get_users(users: &Vec<User>) {
    // iterate over users vector
    for user in users {
        println!(
            "id: {}, username: {}, email: {}",
            user.id, user.username, user.email
        );
    }
}

fn add_user(users: &mut Vec<User>, user: User) {   
    users.push(user);

    println!("\nAfter adding new user:");
    get_users(&users);
}

fn update_user(users: &mut Vec<User>, user: User) {
    for u in users.into_iter() {
        if u.id == user.id {
            // update user
            // clone is used to copy the object
            u.username = user.username.clone();
            u.email = user.email.clone();
        }
    }

    println!("\nAfter updating user:");
    get_users(&users);
}

fn get_user_by_id(users: &Vec<User>, id: &str) -> Result<User, String> {
    for user in users {
        if user.id == id {
            return Ok(user.clone());
        }
    }

    Err(String::from("User not found"))
}

fn delete_user_by_id(users: &mut Vec<User>, id: &str) -> Result<(), String> {
    for (i, user) in users.iter().enumerate() {
        if user.id == id {
            // remove user from users vector by index using remove method
            users.remove(i);
            return Ok(());
        }
    }

    Err(String::from("User not found"))
}
