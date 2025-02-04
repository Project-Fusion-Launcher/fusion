import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";
import type { RouteSectionProps } from "@solidjs/router";
import { WindowEventListener } from "@solid-primitives/event-listener";
import ContextProvider from "./State";

const App = (props: RouteSectionProps) => {
  return (
    <ContextProvider>
      <WindowEventListener onContextmenu={(e) => e.preventDefault()} />
      <WindowTitlebar class="fixed z-50 w-full bg-transparent" />
      <Sidebar />
      <div class="flex w-full flex-col">{props.children}</div>
    </ContextProvider>
  );
};

export default App;
