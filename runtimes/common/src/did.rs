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

use codec::Encode;

use frame_support::dispatch::Weight;
use sp_runtime::DispatchError;

use did::did_details::{DeriveDidCallAuthorizationVerificationType, DidAuthorizedCallOperation, DidAuthorizedCallOperationWithVerificationRelationship, DidVerificationType, DidVerifiableIdentifier};

pub struct DidCallProxy<Config>(sp_std::marker::PhantomData<Config>);
impl<T: did::Config> did::DidCallProxy<T> for DidCallProxy<T> {
	fn weight(did_call: &DidAuthorizedCallOperation<T>) -> Weight {
		todo!()
	}

	fn authorise(
		did_call: &DidAuthorizedCallOperation<T>,
		signature: &did::DidSignature,
	) -> Result<(), DispatchError> {
		let verification_key_relationship = did_call
			.call
			.derive_verification_key_relationship()
			.map_err(did::Error::<T>::from)?;

		match verification_key_relationship {
			DidVerificationType::Inline => {
				did_call.did
					.verify_and_recover_signature(&did_call.encode(), &signature)
					.map_err(did::Error::<T>::from).map_err(DispatchError::from)?;
				Ok(())
			}
			DidVerificationType::StoredVerificationKey(key_relationship) => {
				let wrapped_operation = DidAuthorizedCallOperationWithVerificationRelationship {
					operation: did_call.clone(),
					verification_key_relationship: key_relationship,
				};

				did::Pallet::<T>::verify_did_operation_signature_and_increase_nonce(&wrapped_operation, &signature)
					.map_err(did::Error::<T>::from).map_err(DispatchError::from)
			}
		}
	}
}
