//! Implementations for making requests for Accounts (Client API).

use pteroxide_models::{
    client::account::{Account, ApiKey, TwoFactorWrapper},
    fractal::{FractalData, FractalList},
};
use serde_json::json;

use crate::client::Client;
use crate::errors::Error;
use crate::requests::RequestBuilder;

pub struct GetAccount<'a> {
    http: &'a Client,
}

impl<'a> GetAccount<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Executes a request and returns the [`Account`] if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<Account, Error> {
        match self.http.request::<FractalData<Account>>(
            RequestBuilder::new("/api/client/account")
        ).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}

/// Gets the API keys associated with the account.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     client.get_api_keys()
///         .exec()
///         .await
///         .expect("couldn't get api keys")
///         .iter()
///         .for_each(|k| println!("{}", k.identifier));
/// }
/// ```
pub struct GetApiKeys<'a> {
    http: &'a Client,
}

impl<'a> GetApiKeys<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Executes a request and returns a list of [`ApiKey`]s if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<Vec<ApiKey>, Error> {
        match self.http.request::<FractalList<ApiKey>>(
            RequestBuilder::new("/api/client/account/api-keys")
        ).await {
            Ok(v) => Ok(v.unwrap()
                .data
                .iter()
                .map(|k| k.attributes.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }
}

/// Create an API key for the account.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     let key = client.create_api_key()
///         .description("my api key")
///         .ip("172.18.0.1".to_string())
///         .exec()
///         .await
///         .expect("couldn't create api key");
/// 
///     println!("{}", key.identifier);
/// }
/// ```
pub struct CreateApiKey<'a> {
    http: &'a Client,
    description: String,
    allowed_ips: Vec<String>,
}

impl<'a> CreateApiKey<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self {
            http,
            description: Default::default(),
            allowed_ips: Default::default(),
        }
    }

    /// Sets the description of the API key.
    /// 
    /// **Note:** this is required and will throw an [`Error`] at execution if empty.
    pub fn description(mut self, description: String) -> Self {
        self.description = description;

        self
    }

    /// Adds a single IP to be bound to the API key.
    pub fn ip(mut self, ip: String) -> Self {
        self.allowed_ips.push(ip);

        self
    }

    /// Sets a list of IPs to be bound to the API key.
    pub fn ips(mut self, ips: Vec<String>) -> Self {
        self.allowed_ips = ips;

        self
    }

    /// Executes a request and returns the new [`ApiKey`] if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`FieldError`] if the `description` is not set.
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`FieldError`]: crate::errors::ErrorKind::FieldError
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<ApiKey, Error> {
        if self.description.is_empty() {
            return Err(Error::from("api key description is required"));
        }

        let mut req = RequestBuilder::new("/api/client/account/api-keys");
        req.method("POST")?;
        req.json(json!({
            "description": self.description,
            "allowed_ips": self.allowed_ips
        }));

        match self.http.request::<FractalData<ApiKey>>(req).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}

/// Deletes a specified API key from the account.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     client.delete_api_key("ATvnaMZwaQgoxplo")
///         .exec()
///         .await
///         .expect("couldn't delete api key");
/// }
/// ```
pub struct DeleteApiKey<'a> {
    http: &'a Client,
    id: String,
}

impl<'a> DeleteApiKey<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id }
    }

    /// Executes a request to delete a specified [`ApiKey`].
    ///
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<(), Error> {
        let mut req = RequestBuilder::new(
            &format!("/api/client/account/api-keys/{}", self.id)
        );
        req.method("DELETE")?;

        match self.http.request::<()>(req).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Gets a two-factor authentication code to setup 2FA for the account.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     let url = client.get_two_factor_code()
///         .exec()
///         .await
///         .expect("couldn't get 2fa code");
/// 
///     println!("{:?}", url);
/// }
/// ```
pub struct GetTwoFactorCode<'a> {
    http: &'a Client,
}

impl<'a> GetTwoFactorCode<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Executes a request and returns the 2FA authentication code if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<String, Error> {
        match self.http.request::<TwoFactorWrapper>(
            RequestBuilder::new("/api/client/account/two-factor")
        ).await {
            Ok(v) => Ok(v.unwrap().data.image_url_data),
            Err(e) => Err(e),
        }
    }
}

/// Updates several fields on the account. This can be the email, password, or two-factor status.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     client.update_account()
///         .email(
///             "myEmail@example.com".to_string(),
///             "my_password".to_string(),
///         )
///         .password(
///             "old_password".to_string(),
///             "newPassword".to_string(),
///         )
///         .unwrap()
///         .exec()
///         .await
///         .expect("couldn't update account");
/// }
/// ```
pub struct UpdateAccount<'a> {
    http: &'a Client,
    email: Option<(String, String)>,
    password: Option<(String, String)>,
}

impl<'a> UpdateAccount<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self {
            http,
            email: None,
            password: None,
        }
    }

    /// Sets the email to be updated on the account.
    pub fn email(mut self, new: String, pass: String) -> Self {
        self.email = Some((new, pass));

        self
    }

    /// Sets the password to be updated on the account.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`FieldError`] if the new password is not unique.
    /// 
    /// [`FieldError`]: crate::errors::ErrorKind::FieldError
    pub fn password(mut self, old: String, new: String) -> Result<Self, Error> {
        if old == new {
            return Err(Error::from("cannot update password with the same value"));
        }
        self.password = Some((old, new));

        Ok(self)
    }

    /// Executes a request to update the email and/or password on the account.
    ///
    /// ## Errors
    /// Returns an [`Error`] with the kind [`FieldError`] if either the update email field or
    /// update password field is not specified.
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`FieldError`]: crate::errors::ErrorKind::FieldError
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<(), Error> {
        if self.email.is_none() && self.password.is_none() {
            return Err(Error::from("cannot update the account with no fields"));
        }

        if let Some(data) = self.email {
            let mut req = RequestBuilder::new("/api/client/account/email");
            req.method("PUT")?;
            req.json(json!({
                "email": data.0,
                "password": data.1
            }));

            match self.http.request::<()>(req).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        if let Some(data) = self.password {
            let mut req = RequestBuilder::new("/api/client/account/password");
            req.method("PUT")?;
            req.json(json!({
                "current_password": data.0,
                "password": data.1,
                "password_confirmation": data.1
            }));

            match self.http.request::<()>(req).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
