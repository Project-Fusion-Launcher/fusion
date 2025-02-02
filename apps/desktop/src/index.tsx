/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import "./styles.css";
import "@repo/ui/fonts";
import { HashRouter, Navigate, Route } from "@solidjs/router";
import Library from "./routes/Library/Library";
import Retro from "./routes/Retro";
import Downloads from "./routes/Downloads/Downloads";
import Settings from "./routes/Settings/Settings";
import StorefrontsSettings from "./routes/Settings/pages/StorefrontsSettings/StorefrontsSettings";
import GeneralSettings from "./routes/Settings/pages/GeneralSettings";
import DownloadsSettings from "./routes/Settings/pages/DownloadsSettings";

render(
  () => (
    <HashRouter root={App}>
      <Route path={["/", "/library"]} component={Library} />
      <Route path="/retro" component={Retro} />
      <Route path="/downloads" component={Downloads} />
      <Route path="/settings" component={Settings}>
        <Route path="/" component={() => <Navigate href={"storefronts"} />} />
        <Route path="/storefronts" component={StorefrontsSettings} />
        <Route path="/general" component={GeneralSettings} />
        <Route path="/downloads" component={DownloadsSettings} />
      </Route>
    </HashRouter>
  ),
  document.getElementById("root") as HTMLElement,
);
