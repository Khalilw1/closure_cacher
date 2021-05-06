//! Closure Cacher abstracts memoization on provided closure by storing
//! its data in a hashmap and returning references to its output
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

/// Cacher Struct that allows provision of a closure and 
/// memoizes its output
/// # Examples
/// ```
/// use closure_cacher::Cacher;
/// let mut cacher = Cacher::new(|x| x + 1);
/// assert_eq!(cacher.get(&1), &2);
/// ```
pub struct Cacher<I, O, T>
where 
    T: Fn(I) -> O,
{
    calc: T,
    cache: HashMap<I, O>
}


impl<I: Eq + Hash + Copy, O, T> Cacher<I, O, T> 
where
    T: Fn(I) -> O
{
    /// Creates new Cacher instance
    pub fn new(calc: T) -> Cacher<I, O, T> {
        Cacher {
            calc,
            cache: HashMap::new()
        }
    }

    /// Get value by applying the cacher provided closure
    pub fn get(&mut self, n: &I) -> &O {
        self.cache.entry(*n).or_insert((self.calc)(*n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_closure_output() {
        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.get(&5), &6);
    }
}
