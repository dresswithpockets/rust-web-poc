use crate::model::user::user_server::User;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self {}
    }
}

impl User for UserService {

}