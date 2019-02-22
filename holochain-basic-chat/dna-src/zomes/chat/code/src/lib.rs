#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
    hash::HashString,
    cas::content::Address,
    json::{JsonString},
    error::HolochainError,
};

mod anchor;
mod message;
mod stream;
mod member;
mod utils;

define_zome! {

	entries: [
		message::message_definition(),
    	stream::public_stream_definition(),
        member::profile_definition(),
        anchor::anchor_definition()
	]

    genesis: || {
        {
    		Ok(())
        }
    }

	functions: [
		register: {
			inputs: | name: String, avatar_url: String |,
			outputs: |result: ZomeApiResult<Address>|,
			handler: member::handlers::handle_register
		}
		create_stream: {
			inputs: |name: String, description: String, initial_members: Vec<Address>|,
			outputs: |result: ZomeApiResult<Address>|,
			handler: stream::handlers::handle_create_stream
		}
		join_stream: {
		    inputs: |stream_address: HashString|,
		    outputs: |result: ZomeApiResult<()>|,
		    handler: stream::handlers::handle_join_stream
		}
		get_all_public_streams: {
			inputs: | |,
			outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<stream::Stream>>|,
			handler: stream::handlers::handle_get_all_public_streams
		}
		get_members: {
			inputs: |stream_address: HashString|,
			outputs: |result: ZomeApiResult<Vec<Address>>|,
			handler: stream::handlers::handle_get_members
		}
		get_member_profile: {
			inputs: |agent_address: HashString|,
			outputs: |result: ZomeApiResult<member::Profile>|,
			handler: member::handlers::handle_get_member_profile			
		}
		get_my_member_profile: {
			inputs: | |,
			outputs: |result: ZomeApiResult<member::Profile>|,
			handler: member::handlers::handle_get_my_member_profile			
		}
		post_message: {
			inputs: |stream_address: HashString, message: message::MessageSpec|,
			outputs: |result: ZomeApiResult<()>|,
			handler: stream::handlers::handle_post_message
		}
		get_messages: {
			inputs: |address: HashString|,
			outputs: |result: ZomeApiResult<utils::GetLinksLoadResult<message::Message>>|,
			handler: stream::handlers::handle_get_messages
		}
	]

	 traits: {
	        hc_public [
	        	register,
	        	create_stream,
	        	join_stream,
	        	get_all_public_streams,
	        	get_members,
	        	get_member_profile,
	        	get_my_member_profile,
	        	post_message,
	        	get_messages
	        ]
	}
 }


