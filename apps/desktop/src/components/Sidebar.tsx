import { Tabs } from "@kobalte/core";

const Sidebar = () => {
    return (
        <Tabs.Root
            orientation="vertical"
            class="bg-slate-500 h-full w-72 bg-glow-conic"
        >
            <Tabs.List />
        </Tabs.Root>
    );
};

export default Sidebar;
