// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use frame_support::storage::bounded_btree_set::BoundedBTreeSet;
use kilt_support::deposit::Deposit;
use sp_runtime::{traits::Zero, AccountId32, SaturatedConversion};
use sp_std::{
	collections::btree_set::BTreeSet,
	convert::{TryFrom, TryInto},
	vec::Vec,
};

use crate::{
	did_details::{DidCreationDetails, DidDetails, DidEncryptionKey, DidVerificationKey},
	errors::StorageError,
	service_endpoints::{DidEndpoint, ServiceEndpointType, ServiceEndpointUrl},
	AccountIdOf, BlockNumberOf, Config, DidIdentifierOf,
};

pub(crate) fn get_key_agreement_keys<T: Config>(
	n_keys: u32,
) -> BoundedBTreeSet<DidEncryptionKey, T::MaxTotalKeyAgreementKeys> {
	BoundedBTreeSet::try_from(
		(1..=n_keys)
			.map(|i| {
				// Converts the loop index to a 32-byte array;
				let mut seed_vec = i.to_be_bytes().to_vec();
				seed_vec.resize(32, 0u8);
				let seed: [u8; 32] = seed_vec
					.try_into()
					.expect("Failed to create encryption key from raw seed.");
				DidEncryptionKey::X25519(seed)
			})
			.collect::<BTreeSet<DidEncryptionKey>>(),
	)
	.expect("Failed to convert key_agreement_keys to BoundedBTreeSet")
}

pub(crate) fn get_service_endpoints<T: Config>(
	count: u32,
	endpoint_id_length: u32,
	endpoint_type_count: u32,
	endpoint_type_length: u32,
	endpoint_url_count: u32,
	endpoint_url_length: u32,
) -> Vec<DidEndpoint<T>> {
	(0..count)
		.map(|i| {
			let mut endpoint_id = i.to_be_bytes().to_vec();
			endpoint_id.resize(endpoint_id_length.saturated_into(), 0u8);
			let endpoint_types = (0..endpoint_type_count)
				.map(|t| {
					let mut endpoint_type = t.to_be_bytes().to_vec();
					endpoint_type.resize(endpoint_type_length.saturated_into(), 0u8);
					endpoint_type
				})
				.collect();
			let endpoint_urls = (0..endpoint_url_count)
				.map(|u| {
					let mut endpoint_url = u.to_be_bytes().to_vec();
					endpoint_url.resize(endpoint_url_length.saturated_into(), 0u8);
					endpoint_url
				})
				.collect();
			DidEndpoint::new(endpoint_id, endpoint_types, endpoint_urls)
		})
		.collect()
}

pub(crate) fn generate_base_did_creation_details<T: Config>(
	did: DidIdentifierOf<T>,
	submitter: AccountIdOf<T>,
) -> DidCreationDetails<T> {
	DidCreationDetails { did, submitter }
}

pub(crate) fn generate_base_did_details<T>(authentication_key: DidVerificationKey) -> DidDetails<T>
where
	T: Config,
	<T as frame_system::Config>::AccountId: From<AccountId32>,
{
	DidDetails::new(
		authentication_key,
		BlockNumberOf::<T>::default(),
		Deposit {
			owner: AccountId32::new([0u8; 32]).into(),
			amount: Zero::zero(),
		},
	)
	.expect("Failed to generate new DidDetails from auth_key due to BoundedBTreeSet bound")
}

// Written this way to not break anything else. Might be refactored into a
// function if needed.
impl<T: Config> DidDetails<T> {
	/// Add new key agreement keys to the DID.
	///
	/// The new keys are added to the set of public keys.
	pub fn add_key_agreement_keys(
		&mut self,
		new_key_agreement_keys: BoundedBTreeSet<DidEncryptionKey, <T as Config>::MaxTotalKeyAgreementKeys>,
		block_number: BlockNumberOf<T>,
	) -> Result<(), StorageError> {
		for new_key_agreement_key in new_key_agreement_keys {
			self.add_key_agreement_key(new_key_agreement_key, block_number)?;
		}

		Ok(())
	}
}

// Written this way to not break anything else. Might be refactored into a
// function if needed.
impl<T: Config> DidEndpoint<T> {
	pub(crate) fn new(id: Vec<u8>, types: Vec<Vec<u8>>, urls: Vec<Vec<u8>>) -> Self {
		let bounded_id = id.try_into().expect("Service ID too long.");
		let bounded_types = types
			.iter()
			.map(|el| el.to_vec().try_into().expect("Service type too long."))
			.collect::<Vec<ServiceEndpointType<T>>>()
			.try_into()
			.expect("Too many types for the given service.");
		let bounded_urls = urls
			.iter()
			.map(|el| el.to_vec().try_into().expect("Service URL too long."))
			.collect::<Vec<ServiceEndpointUrl<T>>>()
			.try_into()
			.expect("Too many URLs for the given service.");

		Self {
			id: bounded_id,
			service_types: bounded_types,
			urls: bounded_urls,
		}
	}
}
