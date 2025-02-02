import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";
import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";
import { createEffect } from "solid-js";
import { WindowEventListener } from "@solid-primitives/event-listener";
import ContextProvider from "./State";
import { pages } from "./models/pages";

const App = (props: RouteSectionProps) => {
  const initialTab = () =>
    props.location.pathname.split("/")[1] || pages.library.name;

  const [selectedTab, setSelectedTab] = createSignal(initialTab());

  const navigate = useNavigate();
  const location = useLocation();

  const handleTabChange = (tab: string) => {
    setSelectedTab(tab);
    navigate("/" + tab);
  };

  // Update the current tab when the URL changes (webview navigation)
  createEffect(() => {
    const path = location.pathname.split("/")[1];
    if (path && path !== selectedTab()) {
      setSelectedTab(path);
    }
  });

  return (
    <ContextProvider>
      <WindowEventListener onContextmenu={(e) => e.preventDefault()} />
      <WindowTitlebar class="fixed z-50 w-full bg-transparent" />
      <Sidebar selectedTab={selectedTab()} onTabChange={handleTabChange} />
      <div class="flex w-full flex-col">{props.children}</div>
    </ContextProvider>
  );
};

export default App;
