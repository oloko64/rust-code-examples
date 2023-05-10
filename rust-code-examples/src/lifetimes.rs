// This makes sure that the referenced array and the referenced string are not dropped.
pub fn append_arr<'a>(vec: &mut Vec<&'a str>, append: &'a str) {
    vec.push(append)
}

// === Advanced Lifetimes ===
// For more info: https://www.youtube.com/watch?v=MSi3E5Z8oRw

pub fn advanced_lifetimes_example() {
    let arr = vec![1, 2, 3, 4, 5];
    let ref_arr = MyIterator { value: &arr[..] };

    for i in ref_arr {
        print!("{}", i);
    }
    println!();

    // ----------------------------------------------------------------

    let mut arr = vec![1, 2, 3, 4, 5];
    let ref_arr = MyMutableIterator {
        value: &mut arr[..],
    };

    for i in ref_arr {
        *i += 1;
        print!("{}", i);
    }
    println!();
}

struct MyIterator<'a, T> {
    value: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    // In this `next` we have a inferred lifetime, let's call it 'next
    // This code is exactly the same as the one below
    // fn next<'next>(&'next mut self) -> Option<Self::Item> {
    fn next(&mut self) -> Option<Self::Item> {
        // We split the slice into a tuple of the first element and the rest
        let (first, last) = self.value.split_first()?;

        // Then we replace the pointer of the slice with the rest of the slice
        self.value = last;

        // And finally we return the first element
        Some(first)
    }
}

// ----------------------------------------------------------------

struct MyMutableIterator<'a, T> {
    value: &'a mut [T],
}

impl<'a, T> Iterator for MyMutableIterator<'a, T> {
    type Item = &'a mut T;

    // In this `next` we have a inferred lifetime, let's call it 'next
    // This code is exactly the same as the one below
    // fn next<'next>(&'next mut self) -> Option<Self::Item> {
    fn next(&mut self) -> Option<Self::Item> {
        // First we create a double pointer to the slice
        let slice = &mut self.value;

        // Then we replace the pointer of the slice with a pointer to an empty slice
        // So now we have a mutable reference with the lifetime of `'a` not the lifetime of `'next`, allowing us to return a mutable reference
        let slice = std::mem::replace(slice, &mut []);
        // Or `take` in this case as well, to replace with a default value
        // let slice = std::mem::take(slice);

        // Then we split the slice into a tuple of the first element and the rest
        let (first, rest) = slice.split_first_mut()?;

        // Then we replace the pointer of the value with the rest of the slice
        self.value = rest;

        // And finally we return the first element
        Some(first)
    }
}
