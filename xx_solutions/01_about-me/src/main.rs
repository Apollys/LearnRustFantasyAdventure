// --- Task 2 ---

struct Book {
    title: String,
    author: String,
}

fn my_favorite_book() -> Book {
    Book { title: String::from("Diaspora"), author: String::from("Greg Egan") }
}

// ---------------

fn main() {
    // Task 1.
    let my_name = "Apollys";
    println!("Hello universe, my name is: {}", my_name);

    // Task 2.
    let favorite_book = my_favorite_book();
    println!(
        "My favorite book is: '{}' by {}",
        favorite_book.title, favorite_book.author
    );
}
