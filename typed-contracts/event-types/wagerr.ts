import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/wagerr';

export interface WagerCreated {
	wager: ReturnTypes.Wager;
}

export interface WagerActive {
	wager: ReturnTypes.Wager;
}

export interface WagerClaimed {
	wager: ReturnTypes.Wager;
}

export interface WagerClaimAccepted {
	wager: ReturnTypes.Wager;
}

export interface WagerClaimRejected {
	wager: ReturnTypes.Wager;
}

export interface WagerCompleted {
	wager: ReturnTypes.Wager;
}

