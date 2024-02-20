import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export type Wager = {
	creator: AccountId,
	bettor: AccountId | null,
	id: string,
	name: string,
	terms: string,
	amount: ReturnNumber,
	totalStake: ReturnNumber,
	status: WagerStatus,
	claimed: boolean,
	claimant: AccountId | null
}

export enum WagerStatus {
	pending = 'Pending',
	active = 'Active',
	completed = 'Completed'
}

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export enum WagerrError {
	paymentError = 'PaymentError',
	wagerActive = 'WagerActive',
	wagerJoinError = 'WagerJoinError',
	wagerNotFound = 'WagerNotFound',
	wagerClaimError = 'WagerClaimError',
	wagerActionError = 'WagerActionError',
	wagerTransferError = 'WagerTransferError'
}

export enum ClaimAction {
	accept = 'Accept',
	reject = 'Reject'
}

