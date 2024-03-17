/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./styles.css";
import { Router } from "@solidjs/router";

render(
    () => <Router root={App} />,
    document.getElementById("root") as HTMLElement,
);
