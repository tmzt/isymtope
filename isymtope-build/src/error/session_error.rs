
#[derive(Debug, Fail)]
pub enum SessionError {
    #[fail(display = "Error getting session value")]
    ValueGetError,
}

pub type SessionResult<T> = ::std::result::Result<T, SessionError>;
