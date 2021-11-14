enum Light {
  Bright,
  Dull,
}

struct Book {
  pages: i32,
  rating: i32,
}

fn display_page_count(book: Book) {
  println!("pages = {:?}", book.pages);
}

fn display_rating(book: Book) {
  println!("rating = {:?}", book.rating);
}

fn display_light(light: &Light) {
  match light {
    Light::Bright => println!("bright"),
    Light::Dull => println!("dull"),
  };
}

fn main() {
  let bright = Light::Bright;
  let dull = Light::Dull;
  display_light(&bright);
  display_light(&dull);

  let book = Book {
    pages: 100,
    rating: 5,
  };

  display_rating(book);

  let book = Book {
    pages: 12,
    rating: 9,
  };

  display_page_count(book);
}
