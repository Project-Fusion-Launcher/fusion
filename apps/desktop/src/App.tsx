import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";

const App = () => {
  return (
    <>
      <WindowTitlebar class="fixed z-50 w-full bg-transparent" />
      <Sidebar />
    </>
  );
};

export default App;
