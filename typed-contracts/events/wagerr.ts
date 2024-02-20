import type * as EventTypes from '../event-types/wagerr';
import type {ContractPromise} from "@polkadot/api-contract";
import type {ApiPromise} from "@polkadot/api";
import EVENT_DATA_TYPE_DESCRIPTIONS from '../event-data/wagerr.json';
import {getEventTypeDescription} from "../shared/utils";
import {handleEventReturn} from "@727-ventures/typechain-types";

export default class EventsClass {
	readonly __nativeContract : ContractPromise;
	readonly __api : ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		api : ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__api = api;
	}

	public subscribeOnWagerCreatedEvent(callback : (event : EventTypes.WagerCreated) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerCreated', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerCreated);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerCreated');
	}

	public subscribeOnWagerActiveEvent(callback : (event : EventTypes.WagerActive) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerActive', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerActive);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerActive');
	}

	public subscribeOnWagerClaimedEvent(callback : (event : EventTypes.WagerClaimed) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerClaimed', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerClaimed);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerClaimed');
	}

	public subscribeOnWagerClaimAcceptedEvent(callback : (event : EventTypes.WagerClaimAccepted) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerClaimAccepted', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerClaimAccepted);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerClaimAccepted');
	}

	public subscribeOnWagerClaimRejectedEvent(callback : (event : EventTypes.WagerClaimRejected) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerClaimRejected', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerClaimRejected);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerClaimRejected');
	}

	public subscribeOnWagerCompletedEvent(callback : (event : EventTypes.WagerCompleted) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('WagerCompleted', EVENT_DATA_TYPE_DESCRIPTIONS)) as EventTypes.WagerCompleted);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'WagerCompleted');
	}


	private __subscribeOnEvent(
		callback : (args: any[], event: any) => void,
		filter : (eventName: string) => boolean = () => true
	) {
		// @ts-ignore
		return this.__api.query.system.events((events) => {
			events.forEach((record: any) => {
				const { event } = record;

				if (event.method == 'ContractEmitted') {
					const [address, data] = record.event.data;

					if (address.toString() === this.__nativeContract.address.toString()) {
						const {args, event} = this.__nativeContract.abi.decodeEvent(data);

						if (filter(event.identifier.toString()))
							callback(args, event);
					}
				}
			});
		});
	}

}