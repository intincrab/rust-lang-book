struct Film {
    title:String,
    director:String,
    studion:String
}

struct  Book {
    title:String,
    author:String,
    publisher:String,
}
struct Album {
    title:String,
    artist:String,
    label:String
}
trait Catalog {
    fn describe(&self){
        print!("We need more info about this.")
    }
    
}
impl Catalog for Album {}

impl Catalog for Book {
    fn describe(&self) {
        print!(
            "{} was written by {} and published by {}",
            self.title,
            self.author,
            self.publisher
        )
    }
    
}

impl Catalog for Film {
    fn describe(&self) {
        print!(
            "{} was directed by {} through {} studios",
            self.title,
            self.director,
            self.studion
        )
    }
    
}

fn main(){
    let capt_marvel =Film {
        title:String::from("Captain Marvel"),
        director:String::from("Anna Boden and Ryan Fleck"),
        studion:String::from("Marvel")
    };

    capt_marvel.describe();

    let elantris =Book {
        title:String::from("Elentris"),
        author:String::from("Brandon Sanderson"),
        publisher:String::from("Tor Books")
    };
    elantris.describe();

    let let_it_be = Album {
        title:String::from("Let it be"),
        artist:String::from("Beatles"),
        label:String::from("Apple")
    };
    let_it_be.describe();
}