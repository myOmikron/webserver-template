import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import React from "react";
import { UserProvider } from "../context/user";

export const Route = createRootRoute({
    component: () => (
        <>
            <UserProvider>
                <Outlet />
            </UserProvider>
            <TanStackRouterDevtools />
        </>
    ),
});
