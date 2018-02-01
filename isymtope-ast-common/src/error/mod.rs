pub mod processing_error;
pub mod session_error;
#[cfg(feature = "types")]
pub mod type_error;

pub use self::processing_error::*;
pub use self::session_error::*;
#[cfg(feature = "types")]
pub use self::type_error::*;

#[macro_export]
macro_rules! reduction_err_bt (
    () => ({ DocumentProcessingError::ReductionErrorWithBacktrace(::failure::Backtrace::new()) })
);

#[macro_export]
macro_rules! try_process_from_err (
    ($s: expr) => ({ DocumentProcessingError::TryProcessFromError($s.into(), ::failure::Backtrace::new()) })
);

#[macro_export]
macro_rules! try_eval_from_err (
    ($s: expr) => ({ DocumentProcessingError::TryEvalFromError($s.into(), ::failure::Backtrace::new()) })
);
