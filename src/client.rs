use crate::models::{LoginResponse};
use crate::user::User;
use super::models::Login;

pub const BASE_URL: &str = "https://www.portal-q-cells.us/phoebus";

pub struct Client {
    pub user: User,
}

impl Client {
    /// Creates a new API-Client with a logged in user
    ///
    /// # Arguments
    ///
    /// * `username`: Username of the user
    /// * `password`: Plain-text password of the user
    ///
    /// returns: Result<Client, Error>
    ///
    /// # Examples
    ///
    /// ```
    /// // Default-Guest user
    /// let client = qcells::Client::new("guest", "solax123456").expect("login failed");
    /// println!("{}", client.user.username);
    /// ```
    pub fn new(username: &str, password: &str) -> Result<Client> {
        let login_response = match login(username, password) {
            Ok(resp) => {
                if !resp.success {
                    return Err(Error { message: "login failed".to_string() });
                }
                resp
            }
            Err(err) => return Err(err),
        };
        return Ok(Client { user: User::from_login(&login_response) });
    }
}

pub(crate) fn login(username: &str, password: &str) -> Result<LoginResponse> {
    let hash = format!("{:x}", md5::compute(password.as_bytes()));
    let login = Login { username: username.to_string(), password: hash };

    let client = reqwest::blocking::Client::new();
    let request = client.post(format!("{}/login/loginNew", BASE_URL))
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
        .form(&login).build();

    let request = match request {
        Ok(req) => req,
        Err(err) => return Err(Error { message: err.to_string() })
    };
    let response = match client.execute(request) {
        Ok(resp) => resp,
        Err(err) => return Err(Error { message: err.to_string() })
    };
    if !response.status().is_success() {
        return Err(Error { message: format!("Received invalid status code {}", response.status().as_u16()) });
    }

    let login_response: reqwest::Result<LoginResponse> = response.json();
    let login_response = match login_response {
        Ok(resp) => resp,
        Err(err) => return Err(Error { message: err.to_string() })
    };
    return Ok(login_response);
}

/// A `Result` alias where the `Err` case is `qcells::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur when processing a `Request`.
#[derive(Debug)]
pub struct Error {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        let client = Client::new(
            "guest",
            "solax123456",
        );
        let client = match client {
            Ok(client) => client,
            Err(err) => panic!("{}", err.message)
        };
        let user = client.user;
        println!("Token is: {}", user.token);
        assert_ne!(user.token, "")
    }
}