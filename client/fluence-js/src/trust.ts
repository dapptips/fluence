/*
 *   MIT License
 *
 *   Copyright (c) 2020 Fluence Labs Limited
 *
 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 *
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 *
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

import {FluenceClient} from "./fluence_client";
import * as PeerId from "peer-id";
import {encode} from "bs58"

const FORMAT = "11";
const VERSION = "1111";

interface Trust {
    issuedFor: string,
    expiresAt: number,
    signature: string,
    issuedAt: number
}

function trustToString(trust: Trust): string {
    return `${trust.issuedFor}\n${trust.signature}\n${trust.expiresAt}\n${trust.issuedAt}`
}

function certificateToString(cert: Certificate): string {
    let certStr = cert.chain.map(t => trustToString(t)).join("\n");
    return `${FORMAT}\n${VERSION}\n${certStr}`
}


interface Certificate {
    chain: Trust[]
}

async function issueRoot(issuedBy: PeerId,
                         forPk: PeerId,
                         expiresAt: number,
                         issuedAt: number,
) {
    if (expiresAt < issuedAt) {
        throw Error("Expiration time should be greater then issued time.")
    }

    let maxDate = new Date(8640000000000000).getTime();

    /*
    let root_trust = Trust::create(root_kp, root_kp.public_key(), root_expiration, issued_at);

        let trust = Trust::create(root_kp, for_pk, expires_at, issued_at);
     */

    let rootTrust = createTrust(issuedBy, issuedBy, maxDate, issuedAt);
    let trust = createTrust(issuedBy, forPk, expiresAt, issuedAt);

    let chain = [rootTrust, trust];

    return {
        chain: chain
    }
}

async function createTrust(forPk: PeerId, issuedBy: PeerId, expiresAt: number, issuedAt: number): Promise<Trust> {
    let buf = toSignMessage(forPk, expiresAt, issuedAt);

    let signature = encode(await issuedBy.privKey.sign(Buffer.from(buf)));

    let forPkStr = encode(forPk.pubKey.bytes);

    return {
        issuedFor: forPkStr,
        expiresAt: expiresAt,
        signature: signature,
        issuedAt: issuedAt
    };
}


async function issue(issuedBy: PeerId,
                     forPk: PeerId,
                     extendCert: Certificate,
                     expiresAt: number,
                     issuedAt: number): Promise<Certificate> {
    if (expiresAt < issuedAt) {
        throw Error("Expiration time should be greater then issued time.")
    }

    let lastTrust = extendCert.chain[extendCert.chain.length - 1];


    if (lastTrust.issuedFor !== encode(issuedBy.pubKey.bytes)) {
        throw Error("Last trust in chain should be same as 'isseuedBy'.")
    }

    let trust = await createTrust(forPk, issuedBy, expiresAt, issuedAt);

    let chain = [...extendCert.chain];
    chain.push(trust);

    return {
        chain: chain
    }
}

function toSignMessage(pk: PeerId, expiresAt: number, issuedAt: number): Uint8Array {
    let buf = new Uint8Array(48);
    let pkEncoded = pk.pubKey.bytes;
    buf.set(pkEncoded, 0);
    buf.set(numToArray(expiresAt), 32);
    buf.set(numToArray(issuedAt), 40);

    return buf
}

function numToArray(n: number): number[] {
    let byteArray = [0, 0, 0, 0, 0, 0, 0, 0];

    for (let index = 0; index < byteArray.length; index++) {
        let byte = n & 0xff;
        byteArray [index] = byte;
        n = (n - byte) / 256;
    }

    return byteArray;
}


class CertGiver {

    client: FluenceClient;

    constructor() {

    }


}