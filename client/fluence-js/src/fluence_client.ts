/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import {Address, createRelayAddress, ProtocolType} from "./address";
import {callToString, FunctionCall, genUUID, makeFunctionCall,} from "./function_call";
import * as PeerId from "peer-id";
import {Services} from "./services";
import Multiaddr from "multiaddr"
import {Subscriptions} from "./subscriptions";
import * as PeerInfo from "peer-info";
import {FluenceConnection} from "./fluence_connection";

export class FluenceClient {
    private readonly selfPeerInfo: PeerInfo;
    readonly selfPeerIdStr: string;

    private connection: FluenceConnection;

    private services: Services = new Services();

    private subscriptions: Subscriptions = new Subscriptions();

    constructor(selfPeerInfo: PeerInfo) {
        this.selfPeerInfo = selfPeerInfo;
        this.selfPeerIdStr = selfPeerInfo.id.toB58String();
    }

    /**
     * Makes call with response from function. Without reply_to field.
     */
    private static responseCall(target: Address, args: any): FunctionCall {
        return makeFunctionCall(genUUID(), target, args, undefined, "response");
    }

    /**
     * Waits a response that match the predicate.
     *
     * @param predicate will be applied to each incoming call until it matches
     */
    waitResponse(predicate: (args: any, target: Address, replyTo: Address) => (boolean | undefined)): Promise<any> {
        return new Promise((resolve, reject) => {
            // subscribe for responses, to handle response
            // TODO if there's no conn, reject
            this.subscribe((args: any, target: Address, replyTo: Address) => {
                if (predicate(args, target, replyTo)) {
                    resolve(args);
                    return true;
                }
                return false;
            });
        });
    }

    /**
     * Send call and wait a response.
     *
     * @param target receiver
     * @param args message in the call
     * @param predicate will be applied to each incoming call until it matches
     */
    async sendCallWaitResponse(target: Address, args: any, predicate: (args: any, target: Address, replyTo: Address) => (boolean | undefined)): Promise<any> {
        await this.sendCall(target, args, true);
        return this.waitResponse(predicate);
    }

    /**
     * Send call and forget.
     *
     * @param target receiver
     * @param args message in the call
     * @param reply add a `replyTo` field or not
     * @param name common field for debug purposes
     */
    async sendCall(target: Address, args: any, reply?: boolean, name?: string) {
        if (this.connection && this.connection.isConnected()) {
            await this.connection.sendFunctionCall(target, args, reply, name);
        } else {
            throw Error("client is not connected")
        }
    }

    /**
     * Send call to the service.
     *
     * @param serviceId
     * @param args message to the service
     * @param name common field for debug purposes
     */
    async sendServiceCall(serviceId: string, args: any, name?: string) {
        if (this.connection && this.connection.isConnected()) {
            await this.connection.sendServiceCall(serviceId, args, name);
        } else {
            throw Error("client is not connected")
        }
    }

    /**
     * Send call to the service and wait a response matches predicate.
     *
     * @param serviceId
     * @param args message to the service
     * @param predicate will be applied to each incoming call until it matches
     */
    async sendServiceCallWaitResponse(serviceId: string, args: any, predicate: (args: any, target: Address, replyTo: Address) => (boolean | undefined)): Promise<any> {
        await this.sendServiceCall(serviceId, args);
        return await this.waitResponse(predicate);
    }

    /**
     * Handle incoming call.
     * If FunctionCall returns - we should send it as a response.
     */
    handleCall(): (call: FunctionCall) => FunctionCall | undefined {

        let _this = this;

        return (call: FunctionCall) => {
            console.log("FunctionCall received:");

            // if other side return an error - handle it
            // TODO do it in the protocol
            /*if (call.arguments.error) {
                this.handleError(call);
            } else {

            }*/

            let target = call.target;

            // the tail of addresses should be you or your service
            let lastProtocol = target.protocols[target.protocols.length - 1];

            // call all subscriptions for a new call
            _this.subscriptions.applyToSubscriptions(call);

            switch (lastProtocol.protocol) {
                case ProtocolType.Service:
                    try {
                        // call of the service, service should handle response sending, error handling, requests to other services
                        let applied = _this.services.applyToService(lastProtocol.value, call);

                        // if the request hasn't been applied, there is no such service. Return an error.
                        if (!applied) {
                            console.log(`there is no service ${lastProtocol.value}`);
                            return FluenceClient.responseCall(call.reply_to, {
                                reason: `there is no such service`,
                                msg: call
                            });
                        }
                    } catch (e) {
                        // if service throw an error, return it to the sender
                        return FluenceClient.responseCall(call.reply_to, {reason: `error on execution: ${e}`, msg: call});
                    }

                    return undefined;
                case ProtocolType.Client:
                    if (lastProtocol.value === _this.selfPeerIdStr) {
                        console.log(`relay call: ${call}`);
                    } else {
                        console.warn(`this relay call is not for me: ${callToString(call)}`);
                        return FluenceClient.responseCall(call.reply_to, {reason: `this relay call is not for me`, msg: call});
                    }
                    return undefined;
                case ProtocolType.Peer:
                    if (lastProtocol.value === this.selfPeerIdStr) {
                        console.log(`peer call: ${call}`);
                    } else {
                        console.warn(`this peer call is not for me: ${callToString(call)}`);
                        return FluenceClient.responseCall(call.reply_to, {reason: `this relay call is not for me`, msg: call});
                    }
                    return undefined;
            }
        }
    }


    /**
     * Sends a call to register the service_id.
     */
    async registerService(serviceId: string, fn: (req: FunctionCall) => void) {
        await this.connection.registerService(serviceId);

        this.services.addService(serviceId, fn)
    }

    // subscribe new hook for every incoming call, to handle in-service responses and other different cases
    // the hook will be deleted if it will return `true`
    subscribe(predicate: (args: any, target: Address, replyTo: Address) => (boolean | undefined)) {
        this.subscriptions.subscribe(predicate)
    }



    /**
     * Sends a call to unregister the service_id.
     */
    async unregisterService(serviceId: string) {
        if (this.services.deleteService(serviceId)) {
            console.warn("unregister is not implemented yet (service: ${serviceId}")
            // TODO unregister in fluence network when it will be supported
            // let regMsg = makeRegisterMessage(serviceId, PeerId.createFromB58String(this.nodePeerId));
            // await this.sendFunctionCall(regMsg);
        }
    }

    /**
     * Establish a connection to the node. If the connection is already established, disconnect and reregister all services in a new connection.
     *
     * @param multiaddr
     */
    async connect(multiaddr: string | Multiaddr): Promise<void> {

        multiaddr = Multiaddr(multiaddr);

        let nodePeerId = multiaddr.getPeerId();

        if (!nodePeerId) {
            throw Error("'multiaddr' did not contain a valid peer id")
        }

        let firstConnection: boolean = true;
        if (this.connection) {
            firstConnection = false;
            await this.connection.disconnect();
        }

        let peerId = PeerId.createFromB58String(nodePeerId);
        let relayAddress = await createRelayAddress(nodePeerId, this.selfPeerInfo.id, true);
        let connection = new FluenceConnection(multiaddr, peerId, this.selfPeerInfo, relayAddress, this.handleCall());

        await connection.connect();

        this.connection = connection;

        // if the client already had a connection, it will reregister all services after establishing a new connection.
        if (!firstConnection) {
            for (let service of this.services.getAllServices().keys()) {
                await this.connection.registerService(service);
            }

        }

    }
}
