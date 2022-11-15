use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::routes::admin::dashboard::get_username;
use crate::session_state::TypedSession;
use crate::utils::{e400, e500, see_other};
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::ExposeSecret;
use secrecy::Secret;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

impl FormData {
    fn parse(self) -> Result<Self, InputFormChangePasswordError> {
        let s = self.new_password.expose_secret();
        let is_too_long = s.graphemes(true).count() > 128;
        let is_too_short = s.graphemes(true).count() < 12;

        if is_too_long || is_too_short {
            Err(InputFormChangePasswordError::ValidationError(format!(
                "{} Password not valid minimum requirment",
                s
            )))
        } else {
            Ok(self)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InputFormChangePasswordError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(e500)?;
    if user_id.is_none() {
        return Ok(see_other("/login"));
    };
    let user_id = user_id.unwrap();

    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(see_other("/admin/password"));
    }
    let input_form = form.0.parse().map_err(e400)?;

    let username = get_username(user_id, &pool).await.map_err(e500)?;

    let credentials = Credentials {
        username,
        password: input_form.current_password,
    };

    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(e500(e).into()),
        };
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::routes::admin::password::post::FormData;
    use claim::{assert_err, assert_ok};
    use fake::{faker::internet::en::Password, Fake};
    use secrecy::Secret;

    #[test]
    fn a_password_longer_than_128_graphemes_is_rejected() {
        let password: String = Password(129..130).fake();
        println!("Fake Password {}", password);
        let form = FormData {
            current_password: Secret::new(password.clone()),
            new_password: Secret::new(password.clone()),
            new_password_check: Secret::new(password.clone()),
        };
        assert_err!(form.parse());
    }

    #[test]
    fn a_password_shorter_than_12_graphemes_is_rejected() {
        let password: String = Password(10..12).fake();
        println!("Fake Password {}", password);
        let form = FormData {
            current_password: Secret::new(password.clone()),
            new_password: Secret::new(password.clone()),
            new_password_check: Secret::new(password.clone()),
        };
        assert_err!(form.parse());
    }

    #[test]
    fn a_password_is_valid() {
        let password: String = Password(15..20).fake();
        println!("Fake Password {}", password);
        let form = FormData {
            current_password: Secret::new(password.clone()),
            new_password: Secret::new(password.clone()),
            new_password_check: Secret::new(password.clone()),
        };
        assert_ok!(form.parse());
    }
}
