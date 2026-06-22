pub(crate) trait Unzip3<T> {
    fn unzip3(self) -> (Vec<T>, Vec<T>, Vec<T>);
}

impl<T> Unzip3<T> for Vec<(T, T, T)> {
    fn unzip3(self) -> (Vec<T>, Vec<T>, Vec<T>) {
        let mut vec1 = Vec::<T>::with_capacity(self.len());
        let mut vec2 = Vec::<T>::with_capacity(self.len());
        let mut vec3 = Vec::<T>::with_capacity(self.len());

        for (index, (x, y, z)) in self.into_iter().enumerate() {
            vec1[index] = x;
            vec2[index] = y;
            vec3[index] = z;
        }

        (vec1, vec2, vec3)
    }
}
