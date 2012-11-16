//mod system;
use system::*;
use fact::*;
use std::*;

fn main() {
    let v = ~[(~"arch", arch()),
              (~"user", user())];
    let m = ~std::map::hash_from_vec(v);
    let facts = ~SubFacts(m);
    // print_facts(facts);
    io::println(facts.to_json().to_str());
}
