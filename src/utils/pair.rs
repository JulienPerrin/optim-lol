use coin_cbc::Col;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// See explanation (1).
trait KeyPair<A, B> {
    /// Obtains the first element of the pair.
    fn a(&self) -> &A;
    /// Obtains the second element of the pair.
    fn b(&self) -> &B;
}

// See explanation (2).
impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

// See explanation (3).
impl<A: Hash, B: Hash> Hash for dyn KeyPair<A, B> + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for dyn KeyPair<A, B> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl<A: Eq, B: Eq> Eq for dyn KeyPair<A, B> + '_ {}

// OP's Table struct
pub struct Table<A: Eq + Hash, B: Eq + Hash> {
    map: HashMap<(A, B), Col>,
}

impl<A: Eq + Hash, B: Eq + Hash> Table<A, B> {
    pub fn new() -> Self {
        Table {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, a: &A, b: &B) -> Col {
        *self.map.get(&(a, b) as &dyn KeyPair<A, B>).unwrap()
    }

    pub fn set(&mut self, a: A, b: B, v: Col) {
        self.map.insert((a, b), v);
    }
}

// Boring stuff below.

impl<A, B> KeyPair<A, B> for (A, B) {
    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        &self.1
    }
}
impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn a(&self) -> &A {
        self.0
    }
    fn b(&self) -> &B {
        self.1
    }
}

//----------------------------------------------------------------

#[derive(Eq, PartialEq, Hash)]
struct A(&'static str);

#[derive(Eq, PartialEq, Hash)]
struct B(&'static str);
