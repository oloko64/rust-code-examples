
// This makes sure that the referenced array and the referenced string are not dropped.
pub fn append_arr<'a>(vec: &mut Vec<&'a str>, append: &'a str) {
    vec.push(append)
}
