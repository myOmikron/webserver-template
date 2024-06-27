import { createLazyFileRoute } from "@tanstack/react-router";
import React from "react";

export const Route = createLazyFileRoute("/")({
    component: () => <RoleGuard />,
});

/** The props for the RoleGuard */
type RoleGuardProps = {};

/**
 * A guard that sets the index route according to the role
 */
function RoleGuard(props: RoleGuardProps) {
    return <div></div>;
}
