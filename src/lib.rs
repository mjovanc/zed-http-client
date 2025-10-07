mod builder;
mod claims;
mod jwt_extension;
mod parser;
mod verifier;

use jwt_extension::JwtExtension;

zed_extension_api::register_extension!(JwtExtension);
