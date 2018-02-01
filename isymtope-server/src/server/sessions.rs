#[cfg(feature = "session_time")]
use time::{get_time, Duration, Timespec};
use std::collections::HashMap;

use isymtope_ast_common::*;
use server::*;

#[derive(Debug, Default)]
pub struct MemorySessions {
    session_map: HashMap<String, MemorySession>,
}

impl Sessions for MemorySessions {
    #[cfg(feature = "session_time")]
    fn create(&mut self, session_id: &str, expires: Option<Duration>) -> SessionResult<()> {
        let created = get_time();
        let session = MemorySession::new(created, expires);

        self.session_map.insert(session_id.to_owned(), session);
        Ok(())
    }

    #[cfg(not(feature = "session_time"))]
    fn create(&mut self, session_id: &str) -> SessionResult<()> {
        let session = MemorySession::new();

        self.session_map.insert(session_id.to_owned(), session);
        Ok(())
    }

    fn validate(&mut self, session_id: &str) -> SessionResult<()> {
        Ok(())
    }

    fn destroy(&mut self, session_id: &str) -> SessionResult<()> {
        Ok(())
    }

    fn execute_action(
        &mut self,
        session_id: &str,
        action_op: &ActionOp<ProcessedExpression>,
    ) -> SessionResult<()> {
        Ok(())
    }
}

// impl Sessions {
//     pub fn get_or_create_session<'s>(&'s mut self, previous_key: Option<&str>) -> IsymtopeServerResult<ReturnedSession<'s>> {

//     pub fn create_empty_session(&self) -> IsymtopeServerResult<Session> {
//         let session_key = self.srs.generate_secure_string(self::SESSIONS_SECURE_STRING_BYTES);
//         let ts = get_time();

//         create_session_with_key(session_key, ts)
//     }

//     #[allow(dead_code)]
//     fn create_session(&self) -> IsymtopeServerResult<(String, Session)> {
//         let session_key = self.srs.generate_secure_string(self::SESSIONS_SECURE_STRING_BYTES);
//         let ts = get_time();

//         create_session_with_key(session_key.clone(), ts)
//             .map(|session| (session_key, session))
//     }

//     pub fn expire_session(&mut self, session_key: &str) -> IsymtopeServerVoidResult {
//         // TODO: Implement expiration mechanism
//         Ok(())
//     }

//     fn allocate_new_session<'s>(&'s mut self) -> IsymtopeServerResult<ReturnedSession<'s>> {
//         let (session_key, item) = self.create_session()?;

//         match self.session_map.entry(session_key.clone()) {
//             Entry::Occupied(o) => {
//                 // This should not happen
//                 panic!(format!("Had previous entry with newly generated session key {:?}", session_key));
//             }

//             Entry::Vacant(v) => {
//                 let entry = v.insert(item);
//                 Ok(ReturnedSession(SessionCreationType::CreatedSession(session_key), entry))
//             }
//         }
//     }

//     #[allow(dead_code)]
//     pub fn get_or_create_session<'s>(&'s mut self, previous_key: Option<&str>) -> IsymtopeServerResult<ReturnedSession<'s>> {
//         if let Some(previous_key) = previous_key {
//             if let Some(session) = self.session_map.get_mut(previous_key) {
//                 return Ok(ReturnedSession(SessionCreationType::ExistingSession, session));
//             };

//             Err(IsymtopeServerError::CannotFindSession)
//         } else {
//             self.allocate_new_session()
//         }
//     }
// }
