import { createMemo, createSignal } from "solid-js";
import Sidebar from "./nav";
import { FiMenu } from "solid-icons/fi";
import { getCorePackages, Package } from "../api/lead";
import { version } from "../api/workspace";
import Display from "./display";

export default function App() {
  const [pkg, setPkg] = createSignal<Package[]>([]);
  const [doc, setDoc] = createSignal<[number, number, number]>([-1, 0, 0]);

  createMemo(() => {
    getCorePackages().then(setPkg);
  });

  return (
    <div class="drawer lg:drawer-open">
      <input id="app-sidebar" type="checkbox" class="drawer-toggle" />
      <div class="drawer-content flex flex-col p-2">
        {/* App Content */}
        <div class="flex items-center text-center">
          <label for="app-sidebar" class="drawer-button lg:hidden btn btn-circle bg-base-200 hover:bg-base-100">
            <FiMenu size={"1.5em"} />
          </label>
          <h1 class="mx-auto pr-[3rem] lg:pr-0">{version()}</h1>
        </div>
        <Display {...{ pkg, doc }} />
      </div>
      <div class="drawer-side">
        <label for="app-sidebar" aria-label="close sidebar" class="drawer-overlay"></label>
        <div class="menu p-4 w-80 min-h-full bg-base-100 text-base-content">
          <Sidebar installed={pkg} {...{ doc, setDoc }} />
        </div>

      </div>
    </div>
  );
}