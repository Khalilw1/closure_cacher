//! Closure Cacher abstracts memoization on provided closure by storing
//! its data in a hashmap and returning references to its output
use std::cmp::Eq;
use std::collections::HashMap;
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
    cache: HashMap<I, O>,
}

impl<I: Eq + Hash + Copy, O, T> Cacher<I, O, T>
where
    T: Fn(I) -> O,
{
    /// Creates new Cacher instance
    pub fn new(calc: T) -> Cacher<I, O, T> {
        Cacher {
            calc,
            cache: HashMap::new(),
        }
    }

    /// Get reference value by applying the cacher provided closure
    pub fn get(&mut self, n: &I) -> &O {
        let calc = &self.calc;
        self.cache.entry(*n).or_insert_with(|| calc(*n))
    }
}

/// RefCacher Struct that allows provision of a closure and
/// memoizes its output using references for its input values
/// # Examples
/// ```
/// use closure_cacher::RefCacher;
/// let mut ref_cacher = RefCacher::new(|x| x + 1);
/// assert_eq!(ref_cacher.get(&1), &2);
/// ```

// TODO(khalil): Is there a better way to reabstracted both caching structures
//               since they are very similar
pub struct RefCacher<'a, I, O, T>
where
    T: Fn(&'a I) -> O,
{
    calc: T,
    cache: HashMap<&'a I, O>,
}

impl<'a, I: Eq + Hash, O, T> RefCacher<'a, I, O, T>
where
    T: Fn(&'a I) -> O,
{
    /// Creates new RefCacher instance
    pub fn new(calc: T) -> RefCacher<'a, I, O, T> {
        RefCacher {
            calc,
            cache: HashMap::new(),
        }
    }

    /// Get reference to value by applying the cacher provided closure
    pub fn get(&mut self, n: &'a I) -> &O {
        let calc = &self.calc;
        self.cache.entry(n).or_insert_with(|| calc(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_closure_output_with_input_copy() {
        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.get(&5), &6);
    }

    #[test]
    fn returns_closure_output_with_input_referenced() {
        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.get(&5), &6);
    }
}
