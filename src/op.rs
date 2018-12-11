macro_rules! op_term {
    ($type: ident, $uname: ident, $lname: ident, $uaname: ident, $laname: ident, $( $member: ident ),*) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS> $uname<$type<RHS, Unit>>
            for $type<T, Unit>
        {
            type Output = $type<Output, Unit>;
            fn $lname(self, rhs: $type<RHS, Unit>) -> Self::Output {
                Self::Output::new( $(self.$member.$lname(rhs.$member),)* )
            }
        }

        impl<'a, T: $uname<&'a RHS, Output = Output>, Unit, Output, RHS>
            $uname<&'a $type<RHS, Unit>> for $type<T, Unit>
        {
            type Output = $type<Output, Unit>;
            fn $lname(self, rhs: &'a $type<RHS, Unit>) -> Self::Output {
                Self::Output::new( $(self.$member.$lname(&rhs.$member),)* )
            }
        }

        impl<T: $uaname<RHS>, Unit, RHS> $uaname<$type<RHS, Unit>> for $type<T, Unit> {
            fn $laname(&mut self, rhs: $type<RHS, Unit>) {
                $( self.$member.$laname(rhs.$member); )*
            }
        }

        impl<'a, T: $uaname<&'a RHS>, Unit, RHS> $uaname<&'a $type<RHS, Unit>>
            for $type<T, Unit>
        {
            fn $laname(&mut self, rhs: &'a $type<RHS, Unit>) {
                $( self.$member.$laname(&rhs.$member); )*
            }
        }
    };
}

macro_rules! op_factor {
    ($type: ident, $uname: ident, $lname: ident, $uaname: ident, $laname: ident, $member: ident) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS> $uname<RHS> for $type<T, Unit> {
            type Output = $type<Output, Unit>;
            fn $lname(self, rhs: RHS) -> Self::Output {
                Self::Output::new(self.$member.$lname(rhs))
            }
        }
        impl<T: $uaname<RHS>, Unit, RHS> $uaname<RHS> for $type<T, Unit> {
            fn $laname(&mut self, rhs: RHS) {
                self.$member.$laname(rhs);
            }
        }
    };

    ($type: ident, $uname: ident, $lname: ident, $uaname: ident, $laname: ident, $member: ident, $( $members: ident ),+) => {
        impl<T: $uname<RHS, Output = Output>, Unit, Output, RHS: Copy> $uname<RHS> for $type<T, Unit> {
            type Output = $type<Output, Unit>;
            fn $lname(self, rhs: RHS) -> Self::Output {
                Self::Output::new( self.$member.$lname(rhs), $(self.$members.$lname(rhs),)* )
            }
        }
        impl<T: $uaname<RHS>, Unit, RHS: Copy> $uaname<RHS> for $type<T, Unit> {
            fn $laname(&mut self, rhs: RHS) {
                self.$member.$laname(rhs);
                $( self.$members.$laname(rhs); )*
            }
        }
    };
}
