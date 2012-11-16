extern mod std;

// A union type for nodes or leaves
enum Fact {
    StringFact(~str),
    IntFact(int),
    PathFact(Option<Path>),
    SubFacts(~std::map::HashMap<~str,~Fact>)
}

type Facts = std::map::HashMap<~str,~Fact>;

fn print_pfx(pfx: ~[~str]) {
    for pfx.each |v| {
        io::print(*v);
        io::print(" ");
    };
}

fn print_fact(fact: &Fact, pfx: ~[~str]) {
    match fact {
        &StringFact(v) => { 
            print_pfx(pfx);
            io::println(v); },
        &IntFact(v) => { 
            print_pfx(pfx);
            io::println(v.to_str());},
        &PathFact(v) => {
            match v {
                None => return,
                Some(ref p) => { 
                    print_pfx(pfx);
                    io::println(p.to_str())},
            }
        },
        &SubFacts(m) => {
            for m.each |k, v| {
                print_fact(v, pfx + [k]);}
        }
    }
}

fn print_facts (fact: &Fact) {
    print_fact(fact, ~[]);
}

fn map_to_json(m: ~std::map::HashMap<~str,~Fact>) -> std::json::Json {
    let mut d = send_map::linear::LinearMap();
    for m.each_ref |key, value| {
        d.insert(copy *key, value.to_json());
    }
    std::json::Object(~d)
}

impl Fact: std::json::ToJson {
    fn to_json() -> std::json::Json {
        match self {
            StringFact(v) => {v.to_json() },
            IntFact(v) => {v.to_json() },
            PathFact(v) => {
                match v {
                    None => { let s=@~""; s.to_json() },
                    Some(ref p) => { p.to_str().to_json() },
                }
            },
            SubFacts(m) => {
                map_to_json(m)
            }
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_print() {
        debug!("*** starting test_print");
        let m: &Fact = Fact::IntFact("f",1);
        print_fact(m);
    }
}
