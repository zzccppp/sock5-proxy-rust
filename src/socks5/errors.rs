use std::fmt::{Display, Formatter};
use std::error::Error;
use std::fmt::Result;


/// 版本号不正确
#[derive(Debug)]
pub struct UnexpectedSocksVersionError(pub u8);

impl Display for UnexpectedSocksVersionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "UnexpectedSocksVersionError Occurred.Expect 5 but {}", self.0)
    }
}

impl Error for UnexpectedSocksVersionError {}

/// 数据包长度不正确
#[derive(Debug)]
pub struct UnexpectedDataPackLengthError;

impl Display for UnexpectedDataPackLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "UnexpectedDataPackLengthError Occurred.")
    }
}

impl Error for UnexpectedDataPackLengthError {}

/// 不支持的身份验证方式
#[derive(Debug)]
pub struct UnsupportedIdentifierMethodError;

impl Display for UnsupportedIdentifierMethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "UnsupportedIdentifierMethodError Occurred.")
    }
}

impl Error for UnsupportedIdentifierMethodError {}

/// 数据包数据不正常
#[derive(Debug)]
pub struct UnexpectedDataError;

impl Display for UnexpectedDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "UnexpectedDataError Occurred.")
    }
}

impl Error for UnexpectedDataError {}