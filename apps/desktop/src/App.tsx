import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";
import type { RouteSectionProps } from "@solidjs/router";
import { useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";
import Header from "./components/Header";

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
      <div class="w-full">
        <Header title={currentTab()} />
        {props.children}
      </div>
    </>
  );
};

export default App;
