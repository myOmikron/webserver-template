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


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  FullUser,
} from '../models/index';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    FullUserFromJSON,
    FullUserToJSON,
} from '../models/index';

/**
 * 
 */
export class UsersApi extends runtime.BaseAPI {

    /**
     * Retrieve the currently logged-in user
     * Retrieve the currently logged-in user
     */
    async getMeRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullUser>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/users/me`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullUserFromJSON(jsonValue));
    }

    /**
     * Retrieve the currently logged-in user
     * Retrieve the currently logged-in user
     */
    async getMe(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullUser> {
        const response = await this.getMeRaw(initOverrides);
        return await response.value();
    }

}
