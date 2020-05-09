use std::io::Write;

macro_rules! macro_map {
    ($mcr:ident, $($e:ty),*) => {
        $(
            $mcr!($e);
        )*
    };
}

#[macro_export]
macro_rules! echo {
    ($($e:expr),*) => {
        ($($e),*).echo(&mut std::io::stdout()).unwrap();
        println!();
    };
}

pub trait Echo {
    fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error>;
}

impl<T: Echo> Echo for Vec<T> {
    fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        for (i, value) in self.iter().enumerate() {
            if i > 0 {
                write!(w, " ")?;
            }
            value.echo(w)?;
        }
        Ok(())
    }
}

pub struct Mat<T>(pub Vec<Vec<T>>);

impl<T: Echo> Echo for Mat<T> {
    fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        for i in 0..self.0.len() {
            if i > 0 {
                writeln!(w)?;
            }
            self.0[i].echo(w)?;
        }
        Ok(())
    }
}

pub struct MatS(pub Vec<Vec<char>>);

impl Echo for MatS {
    fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        for i in 0..self.0.len() {
            if i > 0 {
                writeln!(w)?;
            }
            for j in 0..self.0[i].len() {
                write!(w, "{}", self.0[i][j])?;
            }
        }
        Ok(())
    }
}

macro_rules! impl_tuple_for_echo {
    ($v0:ident: $t0:ident $(, $v:ident: $t:ident)*) => {
        impl< $t0 : Echo $(, $t : Echo)* > Echo for ($t0, $($t),*) {
            fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
                let (v0, $($v),*) = self;
                v0.echo(w)?;
                $(
                    write!(w, " ")?;
                    $v.echo(w)?;
                )*
                Ok(())
            }
        }
    };
}

macro_rules! tuple_echo_impls {
    ($v0:ident: $t0:ident $(, $v:ident: $t:ident)*) => {
        tuple_echo_impls!($($v: $t),*);
        impl_tuple_for_echo!($v0: $t0 $(, $v: $t)*);
    };
    () => {};
}

tuple_echo_impls!(
    t7: T7,
    t6: T6,
    t5: T5,
    t4: T4,
    t3: T3,
    t2: T2,
    t1: T1,
    t0: T0
);

macro_rules! impl_echo {
    ($t:ty) => {
        impl Echo for $t {
            fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
                write!(w, "{}", self)
            }
        }
    };
}

macro_rules! impl_echo_float {
    ($t:ty) => {
        impl Echo for $t {
            fn echo(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
                write!(w, "{:.12}", self)
            }
        }
    };
}

macro_map!(impl_echo, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
macro_map!(impl_echo, char, &str, String);
macro_map!(impl_echo_float, f32, f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_echo() {
        fn echo_str(v: impl Echo) -> String {
            let mut buf = vec![];
            v.echo(&mut buf).unwrap();
            String::from_utf8(buf).unwrap()
        }

        assert_eq!(echo_str(1_i32), "1");
        assert_eq!(echo_str((1, 2, 3)), "1 2 3");
        assert_eq!(echo_str((1, 3.14, "hello")), "1 3.140000000000 hello");
        assert_eq!(echo_str(vec![1, 2, 3]), "1 2 3");

        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(echo_str(Mat(m)), "1 2 3\n4 5 6\n7 8 9");

        let m = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        assert_eq!(echo_str(MatS(m)), "abc\ndef\nghi");
    }
}
