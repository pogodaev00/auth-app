use argon2::{PasswordHasher, password_hash::phc::SaltString};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main {}
    }
}

#[component]
fn Main() -> Element {

    let mut toogle_form = use_signal(|| false);
    let mut button_text = use_signal(|| String::new());
    let mut response_msg = use_signal(|| String::new());

    let mut first_name = use_signal(|| String::new());
    let mut last_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {}
}

#[server]
async fn register(
    first_name: String,
    last_name: String,
    email: String,
    password: String
) -> Result<String, ServerFnError> {
    use rusqlite::Connection;

    let conn = Connection::open("./data.db3").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        provider TEXT NOT NULL,
    )", []).unwrap();

    let mut orw = conn.query_row("SELECT email FROM users
        WHERE email = ?1 AND provider = 'classic'", &[&email], |row| {
            row.get::<usize, String>(0)
        });

    let message = match row {
        Err(ref e)        => {
            use argon2::Argon2;

            let argon2 = Argon2::default();
            let password_hash = argon2.hash_password(password.as_bytes())
                .unwrap()
                .to_string();
            println!("{}", password_hash);

            let mut stmt = conn.prepare("INSERT INTO users
                (first_name, last_name, email, password, provider)
                VALUES(?, ?, ?, ?, ?)").unwrap();
            stmt.execute(&[
                &first_name,
                &last_name,
                &email,
                &password,
                "classic"
            ]).unwrap();

            { "User crated" }
        },
        Ok(_) => {
            "A user with this email  already exists."
        }
    };

    Ok("sdf".to_string())
}