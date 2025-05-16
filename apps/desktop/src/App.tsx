import Sidebar from "./components/Sidebar";
import type { RouteSectionProps } from "@solidjs/router";
import { WindowEventListener } from "@solid-primitives/event-listener";
import ContextProvider from "./state/GameContext";

const App = (props: RouteSectionProps) => {
  return (
    <ContextProvider>
      <WindowEventListener onContextmenu={(e) => e.preventDefault()} />
      <Sidebar />
      <div class="relative flex w-full flex-col">{props.children}</div>
    </ContextProvider>
  );
};

export default App;
