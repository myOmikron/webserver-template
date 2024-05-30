import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import { RouterProvider } from "react-router";
import { router } from "./router";
import { LoggingSwitch } from "./utils/console";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <LoggingSwitch />
        <RouterProvider router={router} />
    </React.StrictMode>,
);
