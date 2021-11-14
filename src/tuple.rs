#[macro_export]
macro_rules! tuple_type {
    ($type_name:ident, $size:literal, add, BASE) => {
        impl<T> std::ops::Add for $type_name<T> where T: crate::float::Float {
            type Output = $type_name<T>;

            fn add(self, rhs: Self) -> Self::Output {
                let mut out: $type_name<T> = Default::default();
                for i in 0..$size {
                    out.data[i] = self[i] + rhs[i];
                }
                out
            }
        }
    };
    ($type_name:ident, $size:literal, elementwise_mul, BASE) => {
        impl<T> std::ops::Mul for $type_name<T> where T: crate::float::Float {
            type Output = $type_name<T>;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut out: $type_name<T> = Default::default();
                for i in 0..$size {
                    out.data[i] = self[i] * rhs[i];
                }
                out
            }
        }
    };
    ($type_name:ident, $size:literal, sub, BASE) => {
        tuple_type!($type_name, $size, (sub => output = $type_name), BASE);
    };
    ($type_name:ident, $size:literal, (sub => output = $sub_out_type:ident), BASE) => {
        impl<T> std::ops::Sub for $type_name<T> where T: crate::float::Float {
            type Output = $sub_out_type<T>;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut out: $sub_out_type<T> = Default::default();
                for i in 0..$size {
                    out.data[i] = self[i] - rhs[i];
                }
                out
            }
        }
    };
    ($type_name:ident, $size:literal) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $type_name<T>
        where
            T: crate::float::Float,
        {
            pub data: [T; $size],
        }

        impl<T> std::ops::Index<usize> for $type_name<T>
        where
            T: crate::float::Float,
        {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0..=$size => &self.data[index],
                    _ => panic!("index out of range"),
                }
            }
        }

        impl<T> std::ops::IndexMut<usize> for $type_name<T>
        where
            T: crate::float::Float,
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index]
            }
        }

        impl<T> std::ops::Mul<T> for $type_name<T>
        where
            T: crate::float::Float,
        {
            type Output = $type_name<T>;

            fn mul(self, rhs: T) -> Self::Output {
                let mut out: $type_name<T> = Default::default();
                for i in 0..$size {
                    out.data[i] = self.data[i] * rhs;
                }
                out
            }
        }

        impl<T> std::ops::Div<T> for $type_name<T>
        where
            T: crate::float::Float,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                self * (T::identity() / rhs)
            }
        }

        impl<T> std::ops::Neg for $type_name<T>
        where
            T: crate::float::Float,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self * -T::identity()
            }
        }

        impl<T> crate::utils::FuzzyEq for $type_name<T>
        where
            T: crate::float::Float + crate::utils::FuzzyEq,
        {
            fn fuzzy_eq(&self, other: Self) -> bool {
                for i in 0..$size {
                    if self[i].fuzzy_ne(other[i]) {
                        return false;
                    }
                }
                true
            }
        }
    };
    ($type_name:ident, $size:literal, $($tt:tt),+) => {
           tuple_type!($type_name, $size);
           $(
               tuple_type!($type_name, $size, $tt, BASE);
           )+
       };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_indexing() {
        tuple_type!(Test, 4);

        let t = Test {
            data: [1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!(1.0, t[0]);
        assert_eq!(2.0, t[1]);
        assert_eq!(3.0, t[2]);
        assert_eq!(4.0, t[3]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_indexing_out_of_bounds() {
        tuple_type!(Test, 2);

        let t = Test { data: [1.0, 2.0] };
        let _ = t[2];
    }

    #[test]
    fn test_scalar_mult() {
        tuple_type!(Test, 3);

        let t = Test {
            data: [1.0, 2.0, 3.0],
        };
        let exp = Test {
            data: [3.0, 6.0, 9.0],
        };
        assert_eq!(exp, t * 3.0);
    }

    #[test]
    fn test_div() {
        tuple_type!(Test, 3);

        let t = Test {
            data: [1.0, 2.0, 3.0],
        };
        let exp = Test {
            data: [0.5, 1.0, 1.5],
        };
        assert_eq!(exp, t / 2.0);
    }

    #[test]
    fn test_neg() {
        tuple_type!(Test, 3);

        let t = Test {
            data: [1.0, 2.0, 3.0],
        };
        let exp = Test {
            data: [-1.0, -2.0, -3.0],
        };
        assert_eq!(exp, -t);
    }

    #[test]
    fn test_add() {
        tuple_type!(Test, 4, add);

        let t1 = Test {
            data: [1.0, 2.0, 3.0, 4.0],
        };
        let t2 = Test {
            data: [2.0, 3.0, 4.0, 5.0],
        };
        let exp = Test {
            data: [3.0, 5.0, 7.0, 9.0],
        };
        assert_eq!(exp, t1 + t2)
    }

    #[test]
    fn test_elementwise_mul() {
        tuple_type!(Test, 4, elementwise_mul);

        let t1 = Test {
            data: [1.0, 2.0, 3.0, 4.0],
        };
        let t2 = Test {
            data: [2.0, 3.0, 4.0, -5.0],
        };
        let exp = Test {
            data: [2.0, 6.0, 12.0, -20.0],
        };
        assert_eq!(exp, t1 * t2)
    }

    #[test]
    fn test_sub() {
        tuple_type!(Test, 3, sub);

        let t1 = Test {
            data: [1.0, 2.0, 6.0],
        };
        let t2 = Test {
            data: [2.0, 3.0, 4.0],
        };
        let exp = Test {
            data: [-1.0, -1.0, 2.0],
        };
        assert_eq!(exp, t1 - t2)
    }

    #[test]
    fn test_sub_with_type() {
        tuple_type!(Test, 3);
        tuple_type!(TestWithType, 3, (sub => output = Test));

        let t1 = TestWithType {
            data: [1.0, 2.0, 6.0],
        };
        let t2 = TestWithType {
            data: [2.0, 3.0, 4.0],
        };
        let exp = Test {
            data: [-1.0, -1.0, 2.0],
        };
        assert_eq!(exp, t1 - t2)
    }
}
