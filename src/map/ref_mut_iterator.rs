
/// This is a trait like Iterator but works for mutating the elements being iterated.
/// Of course, this trait can't be used with `for` loops, but can be used like this:
/// ```
///     // iter is something that implements RefMutIterator
///     while let Option::Some(your_elem) = iter.next() {
///         // mutate your_elem
///     }
///     // iter is still an owner of the elements here
/// ```
/// (See tests below).
///
/// The returned type `T` is not `&'a mut T` in case you need to wrap the mutable field and
/// pass other objects by value.
///
/// Meta notes:
/// Surprisingly, this is the simplest way I can make this work, and still feels overcomplicated.
/// As general disclaimer, I'm still learning rust and I don't know what I'm doing, so don't take
/// this as an example of good design.
/// I'm not sure why the signature of Iterator doesn't allow this, but it seems to be about avoiding
/// providing two mutable references to the same element. Many discussions about mutable iterators
/// are very old and I don't know which ones still apply.
pub trait RefMutIterator<'a, T: 'a> {
    fn next(&'a mut self) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use crate::map::ref_mut_iterator::RefMutIterator;

    struct MyList {
        pub vec: Vec<i32>,
    }

    impl MyList {
        pub fn new(n: i32) -> Self {
            let mut vec = Vec::new();
            for i in 0..n {
                vec.push(i);
            }
            MyList { vec }
        }

        pub fn from_iter(iter: MyIter) -> Self {
            MyList {vec: iter.vec }
        }

        pub fn iter_mut(self) -> MyIter {
            MyIter { vec: self.vec, i: -1 }
        }
    }

    struct MyIter {
        pub vec: Vec<i32>,
        i: i32,
    }

    struct IterElem<'a> {
        ref_value: &'a mut i32,
    }

    impl<'a> RefMutIterator<'a, IterElem<'a>> for MyIter {
        fn next(&'a mut self) -> Option<IterElem<'a>> {
            self.i += 1;
            self.vec.get_mut(self.i as usize).map(|i| IterElem { ref_value: i})
        }
    }

    #[test]
    fn basic_ref_mut_iter() {
        let n = 4;
        let my_list = MyList::new(n);
        let mut iter = my_list.iter_mut();
        let mut i = 0;
        while let Option::Some(elem) = iter.next() {
            *elem.ref_value += 10;
            i += 1;
        }
        let updated_list = MyList::from_iter(iter);
        assert_eq!(i, n);
        assert_eq!(updated_list.vec, vec![10, 11, 12, 13]);
    }
}
