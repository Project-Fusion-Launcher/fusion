import { WindowTitlebar } from "@tauri-controls/solid";
import Sidebar from "./components/Sidebar";

const App = () => {
    return (
        <>
            <WindowTitlebar class="fixed w-full z-50 bg-transparent" />
            <Sidebar />
        </>
    );
};

export default App;
