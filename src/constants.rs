// The maximum string buffer size used for serialization purposes.
pub const MAX_STR_BUFFER_SIZE: usize = u8::MAX as usize + 2 /*0.*/ + 1/*-*/ + 1/*align*/;
