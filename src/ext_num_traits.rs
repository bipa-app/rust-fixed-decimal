pub(crate) trait ConstBound {
    const MAX: Self;
    const MIN: Self;
}

pub(crate) trait ConstTen {
    const TEN: Self;
}
pub(crate) trait Ten {
    fn ten() -> Self;
}
pub(crate) trait ExtSigned {
    type Unsigned: num_traits::Unsigned;
}

pub(crate) trait UAbs: ExtSigned {
    fn uabs(self) -> Self::Unsigned;
}

pub(crate) trait Sign {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

pub(crate) trait ILog10: num_traits::Unsigned {
    fn ilog10(self) -> u32;
}

pub(crate) trait NegateIfSigned {
    fn negate_if_signed(self) -> Self;
}
pub(crate) trait IsSigned {
    const IS_SIGNED: bool;
}

macro_rules! num_impl {
    ($tty:ty) => {
        impl ConstBound for $tty {
            const MAX: Self = Self::MAX;
            const MIN: Self = Self::MIN;
        }

        impl ConstTen for $tty {
            const TEN: Self = 10;
        }

        impl Sign for $tty {
            fn is_positive(&self) -> bool {
                self >= &0
            }
        }
    };
}

macro_rules! sign_impl {
    ($tty:ty, $utty:ty) => {
        impl ExtSigned for $tty {
            type Unsigned = $utty;
        }

        impl IsSigned for $tty {
            const IS_SIGNED: bool = true;
        }

        impl UAbs for $tty {
            fn uabs(self) -> Self::Unsigned {
                if self == Self::MIN {
                    (Self::MAX as Self::Unsigned + 1)
                } else {
                    self.abs() as Self::Unsigned
                }
            }
        }

        impl NegateIfSigned for $tty {
            fn negate_if_signed(self) -> Self {
                -self
            }
        }

        num_impl!($tty);
    };
}

macro_rules! unsign_impl {
    ($tty:ty) => {
        impl ExtSigned for $tty {
            type Unsigned = Self;
        }

        impl IsSigned for $tty {
            const IS_SIGNED: bool = false;
        }
        impl UAbs for $tty {
            fn uabs(self) -> Self {
                self
            }
        }

        impl ILog10 for $tty {
            fn ilog10(self) -> u32 {
                self.ilog10()
            }
        }

        impl NegateIfSigned for $tty {
            fn negate_if_signed(self) -> Self {
                self
            }
        }

        num_impl!($tty);
    };
}

sign_impl!(i128, u128);
sign_impl!(i64, u64);
sign_impl!(i32, u32);
sign_impl!(i16, u16);
sign_impl!(i8, u8);
unsign_impl!(u128);
unsign_impl!(u64);
unsign_impl!(u32);
unsign_impl!(u16);
unsign_impl!(u8);

impl<T: ConstTen> Ten for T {
    fn ten() -> Self {
        T::TEN
    }
}
