import React from "react";
import { toast } from "react-toastify";
import { Api } from "../../api/api";
import Form from "../../components/form";
import { useForm } from "@tanstack/react-form";

/**
 * The properties of the login view
 */
type LoginProps = {
    /** The function that should be called on a successful sign-in */
    onLogin(): void;
};

/**
 * The login view
 */
export default function Login(props: LoginProps) {
    const { onLogin } = props;

    const loginForm = useForm({
        defaultValues: {
            username: "",
            password: "",
        },
        onSubmit: async ({ value }) => {
            (await Api.auth.login(value.username, value.password)).match(
                () => onLogin(),
                (err) => toast.error(err.message),
            );
        },
    });

    return (
        <Form onSubmit={loginForm.handleSubmit}>
            <loginForm.Field
                name={"username"}
                children={(field) => (
                    <input
                        name={field.name}
                        value={field.state.value}
                        onChange={(e) => field.handleChange(e.target.value)}
                    />
                )}
            />
            <button type={"submit"}>Login</button>
        </Form>
    );
}
