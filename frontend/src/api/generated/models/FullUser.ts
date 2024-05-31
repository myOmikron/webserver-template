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
 * The full representation for the user
 * @export
 * @interface FullUser
 */
export interface FullUser {
    /**
     * Used for displaying purposes
     * @type {string}
     * @memberof FullUser
     */
    displayName: string;
    /**
     * The identifier of the user
     * @type {string}
     * @memberof FullUser
     */
    uuid: string;
}

/**
 * Check if a given object implements the FullUser interface.
 */
export function instanceOfFullUser(value: object): value is FullUser {
    if (!('displayName' in value) || value['displayName'] === undefined) return false;
    if (!('uuid' in value) || value['uuid'] === undefined) return false;
    return true;
}

export function FullUserFromJSON(json: any): FullUser {
    return FullUserFromJSONTyped(json, false);
}

export function FullUserFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullUser {
    if (json == null) {
        return json;
    }
    return {
        
        'displayName': json['display_name'],
        'uuid': json['uuid'],
    };
}

export function FullUserToJSON(value?: FullUser | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'display_name': value['displayName'],
        'uuid': value['uuid'],
    };
}

