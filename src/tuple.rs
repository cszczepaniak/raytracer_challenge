#[macro_export]
macro_rules! tuple_type {
    ($type_name:ident, $size:literal, add, BASE) => {
        impl std::ops::Add for $type_name {
            type Output = $type_name;

            fn add(self, rhs: Self) -> Self::Output {
                let mut out: $type_name = Default::default();
                for i in 0..$size {
                    out.data[i] = self[i] + rhs[i];
                }
                out
            }
        }
    };
    ($type_name:ident, $size:literal, elementwise_mul, BASE) => {
        impl std::ops::Mul for $type_name {
            type Output = $type_name;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut out: $type_name = Default::default();
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
        impl std::ops::Sub for $type_name {
            type Output = $sub_out_type;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut out: $sub_out_type = Default::default();
                for i in 0..$size {
                    out.data[i] = self[i] - rhs[i];
                }
                out
            }
        }
    };
    ($type_name:ident, $size:literal) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $type_name {
            pub data: [f64; $size],
        }

        impl std::ops::Index<usize> for $type_name {
            type Output = f64;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0..=$size => &self.data[index],
                    _ => panic!("index out of range"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $type_name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index]
            }
        }

        impl std::ops::Mul<f64> for $type_name {
            type Output = $type_name;

            fn mul(self, rhs: f64) -> Self::Output {
                let mut out: $type_name = Default::default();
                for i in 0..$size {
                    out.data[i] = self.data[i] * rhs;
                }
                out
            }
        }

        impl std::ops::Div<f64> for $type_name {
            type Output = $type_name;

            fn div(self, rhs: f64) -> Self::Output {
                1.0 / rhs * self
            }
        }

        impl std::ops::Mul<$type_name> for f64 {
            type Output = $type_name;

            fn mul(self, rhs: $type_name) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Neg for $type_name {
            type Output = $type_name;

            fn neg(self) -> Self::Output {
                -1.0 * self
            }
        }

        impl crate::utils::FuzzyEq for $type_name {
            fn fuzzy_eq(&self, other: Self) -> bool {
                for i in 0..$size{
                    if !self[i].fuzzy_eq(other[i]) {
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
        assert_eq!(exp, 3.0 * t,);
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
