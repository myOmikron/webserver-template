import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { LoggingSwitch } from "./utils/console";
import "react-toastify/dist/ReactToastify.css";
import { ToastContainer } from "react-toastify";
import { RouterProvider, createRouter } from "@tanstack/react-router";

// Import i18n to initialize it
import "./i18n";

// Import the generated route tree
import { routeTree } from "./routeTree.gen";

// Create a new router instance
const router = createRouter({ routeTree });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <LoggingSwitch />
        <ToastContainer />
        <RouterProvider router={router} />
    </React.StrictMode>,
);
