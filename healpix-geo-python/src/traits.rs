pub(crate) trait Unzip3<T1, T2, T3> {
    fn unzip3(self) -> (Vec<T1>, Vec<T2>, Vec<T3>);
}

impl<T1, T2, T3> Unzip3<T1, T2, T3> for Vec<(T1, T2, T3)> {
    fn unzip3(self) -> (Vec<T1>, Vec<T2>, Vec<T3>) {
        let size = self.len();
        let mut vec1 = Vec::<T1>::with_capacity(size);
        let mut vec2 = Vec::<T2>::with_capacity(size);
        let mut vec3 = Vec::<T3>::with_capacity(size);

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
