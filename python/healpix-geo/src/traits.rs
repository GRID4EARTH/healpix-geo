pub(crate) trait Unzip3<T> {
    fn unzip3(self) -> (Vec<T>, Vec<T>, Vec<T>);
}

impl<T> Unzip3<T> for Vec<(T, T, T)> {
    fn unzip3(self) -> (Vec<T>, Vec<T>, Vec<T>) {
        let size = self.len();
        let mut vec1 = Vec::<T>::with_capacity(size);
        let mut vec2 = Vec::<T>::with_capacity(size);
        let mut vec3 = Vec::<T>::with_capacity(size);

        for (x, y, z) in self.into_iter() {
            vec1.push(x);
            vec2.push(y);
            vec3.push(z);
        }

        vec1.shrink_to_fit();
        vec2.shrink_to_fit();
        vec3.shrink_to_fit();

        (vec1, vec2, vec3)
    }
}
