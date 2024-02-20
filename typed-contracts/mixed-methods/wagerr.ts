/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/wagerr';
import type * as ReturnTypes from '../types-returns/wagerr';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";
import DATA_TYPE_DESCRIPTIONS from '../data/wagerr.json';
import EVENT_DATA_TYPE_DESCRIPTIONS from '../event-data/wagerr.json';


export default class Methods {
	readonly __nativeContract : ContractPromise;
	readonly __keyringPair : KeyringPair;
	readonly __callerAddress : string;
	readonly __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* getWager
	*
	* @param { string } id,
	* @returns { Result<ReturnTypes.Wager, ReturnTypes.LangError> }
	*/
	"getWager" (
		id: string,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Wager, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getWager", [id], __options, (result) => { return handleReturnType(result, getTypeDescription(12, DATA_TYPE_DESCRIPTIONS)); });
	}

	/**
	* getActiveWagers
	*
	* @returns { Result<Array<ReturnTypes.Wager>, ReturnTypes.LangError> }
	*/
	"getActiveWagers" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.Wager>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActiveWagers", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, DATA_TYPE_DESCRIPTIONS)); });
	}

	/**
	* getPendingWagers
	*
	* @returns { Result<Array<ReturnTypes.Wager>, ReturnTypes.LangError> }
	*/
	"getPendingWagers" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.Wager>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPendingWagers", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, DATA_TYPE_DESCRIPTIONS)); });
	}

	/**
	* createWager
	*
	* @param { string } name,
	* @param { string } terms,
	* @returns { void }
	*/
	"createWager" (
		name: string,
		terms: string,
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "createWager", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [name, terms], __options);
	}

	/**
	* joinWager
	*
	* @param { string } id,
	* @returns { void }
	*/
	"joinWager" (
		id: string,
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "joinWager", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, EVENT_DATA_TYPE_DESCRIPTIONS);
		}, [id], __options);
	}

}