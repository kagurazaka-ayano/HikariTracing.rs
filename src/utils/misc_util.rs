
pub fn is_rectangular<T>(mat: &Vec<Vec<T>>) -> bool {
    let mut len = mat[0].len();
    for i in 0 .. mat.len() {
        if mat[i].len() != len {
            return false;
        }
        len = mat[i].len();
    }
true
}
