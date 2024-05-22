#![feature(trait_upcasting)]
#![deny(unused_must_use)]
#![allow(unused_imports)]
#![feature(macro_metavar_expr)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(hint_must_use)]
#![allow(unused_variables)]

use std::{
    marker::Unsize,
    ops::CoerceUnsized
    ,
};

pub mod any_value;
pub mod document;
pub mod element;
pub mod global;
pub mod html_div_element;
pub mod html_element;
pub mod js_value;
pub mod node;
pub mod object;
// pub mod prompt;
pub mod text;
pub mod window;
pub mod request_init;
pub mod request;
pub mod response;
pub mod credential_request_options;
pub mod allow_credentials;
pub mod allow_credentials_type;
pub mod attestation_conveyance_preference;
pub mod authentication_extensions_client_inputs;
pub mod authentication_extensions_client_outputs;
pub mod authenticator_assertion_response;
pub mod authenticator_attachment;
pub mod authenticator_attestation_response;
pub mod authenticator_response;
pub mod authenticator_selection_criteria;
pub mod authenticator_transport;
pub mod pub_key_cred_params;
pub mod public_key_credential;
pub mod public_key_credential_creation_options;
pub mod public_key_credential_request_options;
pub mod public_key_credential_rp_entity;
pub mod public_key_credential_user_entity;
pub mod user_verification_requirement;
pub mod navigator;
pub mod credentials_container;
pub mod credential;
pub mod credential_data;
pub mod credential_creation_options;
pub mod html_input_element;
#[cfg(side="server")]
pub mod builder;
pub mod html_form_element;
