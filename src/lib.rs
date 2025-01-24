struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (element, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(element)
    }
}

struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice = &mut self.slice;

        // Using shadowing, temp put an empty slice into self slice
        // and have our temp slice (pointer to pointer) point to the original memory.
        // Temp slice can now be manipulated, splitting off first and rest
        // after which they can be reassigned to our struct slice element
        // Temp slice variable lasts as long as 'iter

        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_vec() {
        let collection = vec![1, 2, 3, 4];
        let wrapper = MyIterator {
            slice: &collection[..],
        };
        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }
    }

    #[test]
    fn test_mutable_vec() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for (_index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;

        }
        assert_eq!(collection.get(0), Some(&2) );
    }
}
