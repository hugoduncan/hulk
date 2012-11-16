// System

extern mod std;
use std::*;
use fact::*;

fn arch() -> ~Fact {
    ~SubFacts(
        ~map::hash_from_vec(
            ~[(~"arch", ~StringFact(os::arch())),
              (~"os-family", ~StringFact(os::family()))]))
}

fn user() -> ~Fact {
    ~SubFacts(
        ~map::hash_from_vec(
            ~[(~"homedir", ~PathFact(os::homedir()))]))
}
