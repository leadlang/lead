import { Setter } from "solid-js";
import { isWorkspace } from "../api/workspace";

interface NavProps {
  set: Setter<string | undefined>
}

export default function Sidebar(_: NavProps) {
  return <>
    <ul class="menu bg-base-200 rounded-box">
      <li>
        <details open>
          <summary><strong>Preinstalled</strong></summary>
          <ul>

          </ul>
        </details>
      </li>
    </ul>
    <ul class="mt-3 menu bg-base-200 rounded-box">
      <li>
        <details open>
          <summary><strong>Workspace {!isWorkspace() && <>(not detected)</>}</strong></summary>
          {isWorkspace() && <></>}
        </details>
      </li>
    </ul>
  </>;
}