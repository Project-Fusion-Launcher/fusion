import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";
import type { RouteSectionProps } from "@solidjs/router";
import { useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

const App = (props: RouteSectionProps) => {
  const [currentTab, setCurrentTab] = createSignal("library");

  const navigate = useNavigate();

  const handleTabChange = (tab: string) => {
    setCurrentTab(tab);
    navigate("/" + tab);
  };

  return (
    <>
      <WindowTitlebar class="fixed z-50 w-full bg-transparent" />
      <Sidebar currentTab={currentTab()} onTabChange={handleTabChange} />
      {props.children}
    </>
  );
};

export default App;
