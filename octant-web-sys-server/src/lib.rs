#![feature(trait_upcasting)]
#![deny(unused_must_use)]
#![allow(unused_imports)]
#![feature(macro_metavar_expr)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(hint_must_use)]
#![allow(unused_variables)]
#![feature(arbitrary_self_types)]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(try_blocks)]
#![feature(impl_trait_in_assoc_type)]
#![allow(dead_code)]

#[cfg(side="client")]
extern crate octant_runtime_client as octant_runtime;
#[cfg(side="server")]
extern crate octant_runtime_server as octant_runtime;

use std::{marker::Unsize, ops::CoerceUnsized};

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
// #[cfg(side = "server")]
// pub mod builder;
pub mod credential;
pub mod credential_creation_options;
pub mod credential_data;
pub mod credential_request_options;
pub mod credentials_container;
#[cfg(side = "client")]
pub mod export;
pub mod html_form_element;
pub mod html_input_element;
#[cfg(side = "client")]
pub mod import;
pub mod navigator;
pub mod pub_key_cred_params;
pub mod public_key_credential;
pub mod public_key_credential_creation_options;
pub mod public_key_credential_request_options;
pub mod public_key_credential_rp_entity;
pub mod public_key_credential_user_entity;
pub mod request;
pub mod request_init;
pub mod response;
pub mod text;
pub mod user_verification_requirement;
pub mod window;
pub mod html_anchor_element;
pub mod location;
pub mod history;
pub mod null;
pub mod event_target;
pub mod event_handler;
pub mod html_u_list_element;
pub mod html_li_element;
pub mod css_style_declaration;
pub mod css_style_sheet;
pub mod style_sheet;
pub mod html_style_element;
pub mod html_head_element;
pub mod dom_token_list;
pub mod html_heading_element;
pub mod html_paragraph_element;
pub mod html_hr_element;
pub mod html_label_element;
pub mod html_br_element;
pub mod js_string;
pub mod attributes;