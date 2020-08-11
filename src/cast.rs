macro_rules! impl_casts_and_cast {
    ($hkt:ident) => {
        pub fn cast<U: en::Num>(self) -> $hkt<U> {
            self.map(en::cast)
        }

        impl_casts!($hkt);
    };
}

macro_rules! impl_casts {
    ($hkt:ident) => {
        pub fn to_i8(self) -> $hkt<i8> {
            self.cast()
        }

        pub fn to_i16(self) -> $hkt<i16> {
            self.cast()
        }

        pub fn to_i32(self) -> $hkt<i32> {
            self.cast()
        }

        pub fn to_i64(self) -> $hkt<i64> {
            self.cast()
        }

        pub fn to_i128(self) -> $hkt<i128> {
            self.cast()
        }

        pub fn to_isize(self) -> $hkt<isize> {
            self.cast()
        }

        pub fn to_u8(self) -> $hkt<u8> {
            self.cast()
        }

        pub fn to_u16(self) -> $hkt<u16> {
            self.cast()
        }

        pub fn to_u32(self) -> $hkt<u32> {
            self.cast()
        }

        pub fn to_u64(self) -> $hkt<u64> {
            self.cast()
        }

        pub fn to_u128(self) -> $hkt<u128> {
            self.cast()
        }

        pub fn to_usize(self) -> $hkt<usize> {
            self.cast()
        }

        pub fn to_f32(self) -> $hkt<f32> {
            self.cast()
        }

        pub fn to_f64(self) -> $hkt<f64> {
            self.cast()
        }
    };
}
