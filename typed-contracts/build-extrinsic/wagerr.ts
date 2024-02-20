/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/wagerr';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	readonly __nativeContract : ContractPromise;
	readonly __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * getWager
	 *
	 * @param { string } id,
	*/
	"getWager" (
		id: string,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getWager", [id], __options);
	}

	/**
	 * getActiveWagers
	 *
	*/
	"getActiveWagers" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getActiveWagers", [], __options);
	}

	/**
	 * getPendingWagers
	 *
	*/
	"getPendingWagers" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPendingWagers", [], __options);
	}

	/**
	 * createWager
	 *
	 * @param { string } name,
	 * @param { string } terms,
	*/
	"createWager" (
		name: string,
		terms: string,
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "createWager", [name, terms], __options);
	}

	/**
	 * joinWager
	 *
	 * @param { string } id,
	*/
	"joinWager" (
		id: string,
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "joinWager", [id], __options);
	}

}