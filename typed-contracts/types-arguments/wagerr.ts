import type BN from 'bn.js';

export type AccountId = string | number[]

export type Wager = {
	creator: AccountId,
	bettor: AccountId | null,
	id: string,
	name: string,
	terms: string,
	amount: (string | number | BN),
	totalStake: (string | number | BN),
	status: WagerStatus
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
	wagerNotFound = 'WagerNotFound'
}

