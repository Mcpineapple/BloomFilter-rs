use bloom_filter::Bloomfilter;

fn main() {
    let mut filter = Bloomfilter::new(50, 4);

    filter.insert("hello".as_bytes());
    filter.insert("cats".as_bytes());
    filter.insert("zerg".as_bytes());

    println!("{}", filter.lookup("zerg".as_bytes()));
    println!("{}", filter.lookup("dogs".as_bytes()));
}
