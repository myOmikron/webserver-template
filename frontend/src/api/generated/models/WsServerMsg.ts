/* tslint:disable */
/* eslint-disable */
/**
 * Frontend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * Websocket messages that originate from the server
 * @export
 * @interface WsServerMsg
 */
export interface WsServerMsg {
}

/**
 * Check if a given object implements the WsServerMsg interface.
 */
export function instanceOfWsServerMsg(value: object): value is WsServerMsg {
    return true;
}

export function WsServerMsgFromJSON(json: any): WsServerMsg {
    return WsServerMsgFromJSONTyped(json, false);
}

export function WsServerMsgFromJSONTyped(json: any, ignoreDiscriminator: boolean): WsServerMsg {
    return json;
}

export function WsServerMsgToJSON(value?: WsServerMsg | null): any {
    return value;
}

