/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import "./styles.css";
import "@repo/ui/fonts";
import { Navigate, Route, Router } from "@solidjs/router";
import Library from "./routes/Library/Library";
import Retro from "./routes/Retro";

render(
  () => (
    <Router root={App}>
      <Route path="/" component={() => <Navigate href={"/library"} />} />
      <Route path="/library" component={Library} />
      <Route path="/retro" component={Retro} />
    </Router>
  ),
  document.getElementById("root") as HTMLElement,
);
