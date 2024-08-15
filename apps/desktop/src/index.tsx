/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./styles.css";
import { Navigate, Route, Router } from "@solidjs/router";
import Library from "./routes/Library";

render(
  () => (
    <Router root={App}>
      <Route path="/" component={() => <Navigate href={"/library"} />} />
      <Route path="/library" component={Library} />
    </Router>
  ),
  document.getElementById("root") as HTMLElement,
);
