use std::ops::Deref;

use enumn::N;
use serde::Deserialize;
use thiserror::Error;

/// Represents the json structure returned by plex.tv
#[derive(Deserialize, Debug, Default)]
pub struct PlexTvError {
  pub code: u32,
  pub message: String,
  pub status: Option<u32>,
}

/// Represents the json structure containing many `PlexTvError`s as returned by plex.tv
#[derive(Deserialize, Debug, Default)]
pub struct PlexTvErrors {
  pub errors: Vec<PlexTvError>,
}

/// A strongly typed enum for API errors with a numeric discriminant.
/// The discriminant is either the code or (code x status), if status is not empty.
#[derive(Debug, Error, N)]
pub enum ApiError {
  #[error("X-Plex-Client-Identifier is missing")]
  ClientIdentifierMissing = 400000,

  #[error("User could not be authenticated")]
  Unauthorized = 401401,

  #[error("The requested resource or endpoint could not be found")]
  NotFound = 404808,

  #[error("API rate limit exceeded")]
  OverRateLimit = 430287,

  #[error(
    "The request was malformed, missed a required parameter or contained invalid values for them"
  )]
  RequestValidationError = 401600,

  #[error("The request method is not allowed for this endpoint")]
  MethodNotAllowed = 407025,

  #[error("None")]
  NotAcceptable = 408436,

  #[error("Internal Server Error. Something went wrong on our end")]
  InternalServerError = 503500,

  #[error("The server could not perform the action required with the entity provided")]
  UnprocessableEntity = 425376,

  #[error("Managed users aren't allowed to perform this action")]
  ForbiddenForRestrictedUsers = 406627,

  #[error("Required field cannot be blank")]
  Blank = 1010,

  #[error("The value provided for the field has already been taken")]
  Taken = 1011,

  #[error("You need a subscription to perform this action")]
  ForbiddenForNonSubscribers = 407836,

  #[error("You need to sign up to perform this action")]
  ForbiddenForAnonymousUsers = 408239,

  #[error("Already Exists")]
  AlreadyExists = 427908,

  #[error("Only Anonymous users can perform this action.")]
  RequiresAnonymousUser = 409045,

  #[error("Code not found or expired")]
  PinNotFoundOrExpired = 1020,

  #[error("Two-Factor authentication is not enabled yet")]
  MfaNotEnabled = 432550,

  #[error("The account has no OTP secret, please visit secret generation endpoint")]
  NoOtpSecret = 432972,

  #[error("Two-Factor authentication already enabled")]
  MfaAlreadyEnabled = 433394,

  #[error("Invalid verification code")]
  MfaInvalid = 412228,

  #[error("Please enter the verification code")]
  MfaRequired = 412629,

  #[error(
    "User could not be authenticated. This account only allows signing in with email address"
  )]
  LoginEmailOnly = 413030,

  #[error("User could not be authenticated. This IP appears to be having trouble signing in to an account (detected repeated failures)")]
  LoginIssues = 413431,

  #[error("Password must be at least 8 characters long")]
  UserPasswordShort = 1032,

  #[error("Password not allowed (too weak)")]
  UserPasswordBlacklisted = 1033,

  #[error("Password cannot be the same as the username or email")]
  UserPasswordSame = 1034,

  #[error("The value for email must be a valid email address")]
  UserEmailInvalid = 1035,

  #[error("A different email address cannot be used as username")]
  UserUsernameDiffersFromEmail = 1036,

  #[error("The username includes invalid characters")]
  UserUsernameInvalidCharacters = 1037,

  #[error("PIN must be 4 digits")]
  UserPinInvalid = 1038,

  #[error("The password provided doesn't match the confirmation")]
  UserPasswordConfirmationConfirmation = 1039,

  #[error("A valid password is required to perform this action")]
  InvalidPassword = 419120,

  #[error("A valid PIN is required to perform this action")]
  InvalidPin = 419523,

  #[error("User is missing the required role to perform this action")]
  MissingRole = 419926,

  #[error("This action is not available for this user")]
  Forbidden = 420329,

  #[error("A valid password is needed to verify the new authentication provider.")]
  NeedsPassword = 420732,

  // Duplicate code * status
  // #[error("Please provide another method of authentication to verify the new authentication provider.")]
  // NeedsVerification = 420732,
  #[error("A valid password and verification code are required to perform this action")]
  InvalidPasswordOrVerificationCode = 421135,

  #[error("A valid password and verification code are required to verify the new authentication provider.")]
  NeedsPasswordAndVerificationCode = 421538,

  #[error("The username is invalid.")]
  InvalidUsername = 441834,

  #[error("The username is unavailable.")]
  UnavailableUsername = 442256,

  #[error("A payment information nonce was not provided")]
  NeedsNonce = 442678,

  #[error("An error happened processing the payment method")]
  PaymentError = 443100,

  #[error("An error happened creating the subscription")]
  SubscriptionError = 443522,

  #[error("An active subscription already exists for this user")]
  AlreadySubscribed = 430268,

  #[error("This action is only available for Plex Professional Installers")]
  ProInstallerOnly = 424359,

  #[error("The plan provided is not valid for this user")]
  InvalidPlan = 424762,

  #[error("The token provided doesn't correspond to a valid discount code")]
  InvalidToken = 445210,

  #[error("The user has already redeemed this discount")]
  CodeAlreadyUsed = 431904,

  #[error("The provider token couldn't be validated with the provider")]
  InvalidProviderToken = 446054,

  #[error("The provider doesn't return an email address, please check the scope of the token.")]
  NoEmailProvider = 446476,

  // Duplicate code * status
  // #[error("The provider doesn't return an email address, please check the scope of the token or if the user already signed up before, they may need to revoke our access so apple can send the email again.")]
  // NoEmailProviderApple = 446476,
  #[error("The provider is already linked.")]
  ProviderAlreadyLinked = 433131,

  #[error("Something went wrong when attempting to manipulate the Cloud Server")]
  CloudServerError = 447320,

  #[error("The auth token for this provider has been revoked by the user and is no longer valid")]
  ProviderRevoked = 435010,

  // Duplicate code * status
  // #[error("The token has expired, please request a new one")]
  // UserResetPasswordTokenExpired = 1062,
  #[error("The token is invalid, please request a new one")]
  UserResetPasswordTokenInvalid = 1062,

  #[error("The password is not valid, please make sure its 8 characters long. If admin, it needs to be 10 characters long with at least 1 digit and a combination of lower and uppercase letters.")]
  UserPasswordInvalid = 1063,

  #[error("The provider is already linked to another account.")]
  ProviderAccountAlreadyLinked = 435176,

  #[error("The settings format is not valid. Make sure is an array of hashes with id, type, value and hidden.")]
  InvalidSettingsFormat = 449430,

  #[error("The subscription end time format is not valid.")]
  InvalidSubscriptionTime = 449852,

  #[error("The Plex Pass subscription is currently set to cancel. You'll need to reactivate it before you can continue.")]
  PlexpassCanceled = 450274,

  #[error("This action is not available for this device.")]
  ForbiddenDevice = 430404,

  #[error("The code provided expired")]
  CodeExpired = 451118,

  #[error("The request timed out or failed and the provider token couldn't be validated with the provider")]
  ProviderNetworkError = 451540,

  #[error("The value for url must be a valid URL")]
  WebhookUrlInvalid = 1071,

  #[error("No suitable anonymous user found for the given anonymous token")]
  InvalidAnonymousToken = 452384,

  #[error("Unable to update a certificate. No certificate exists.")]
  EnablingHttpsWithoutCertificate = 429200,

  #[error("User lacks some requirements for this subscription.")]
  RequirementsForSubscriptionNotMet = 453228,

  #[error("Unable to read user data.")]
  InvalidUserData = 453650,

  #[error("The plan provided conflicts with the user's other existing plan.")]
  PlanConflicts = 454072,

  #[error("The consent format is not valid. Make sure language is a 2-letters string and vendors is an array of hashes with an integer id and a boolean consent.")]
  InvalidConsentFormat = 454494,

  #[error("A valid server token is required to perform this action")]
  ServerTokenRequired = 435240,

  #[error("Code already exists for that specific campaign")]
  PromoCodeTaken = 445810,

  #[error("One or more validation errors prevented the action from being performed")]
  GenericValidationError = 843156,

  #[error("None")]
  GenericRequestError = 799600,

  #[error("The server cannot handle the request")]
  ServiceUnavailable = 509539,

  #[error("Invalid CSR")]
  InvalidCsr = 430440,
}

#[derive(Debug, Default)]
pub struct ApiErrors {
  pub errors: Vec<ApiError>,
}

impl TryFrom<&PlexTvError> for ApiError {
  type Error = ();

  fn try_from(value: &PlexTvError) -> Result<Self, Self::Error> {
    let internal_code = match value.status {
      Some(status) => value.code * status,
      None => value.code,
    };
    ApiError::n(internal_code).ok_or(())
  }
}

impl Deref for ApiErrors {
  type Target = Vec<ApiError>;

  fn deref(&self) -> &Self::Target {
    &self.errors
  }
}

impl Deref for PlexTvErrors {
  type Target = Vec<PlexTvError>;

  fn deref(&self) -> &Self::Target {
    &self.errors
  }
}

impl FromIterator<ApiError> for ApiErrors {
  fn from_iter<T: IntoIterator<Item = ApiError>>(iter: T) -> Self {
    Self {
      errors: Vec::from_iter(iter),
    }
  }
}

#[derive(Debug, Error)]
pub enum InternalClientError {
  #[error("Could not parse service item {0}")]
  UnparseableService(String),
}
