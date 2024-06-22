import { Accessor, Setter } from "solid-js";
import { isWorkspace } from "../api/workspace";
import { Package } from "../api/lead";
import { BsCodeSlash } from "solid-icons/bs";

interface NavProps {
  installed: Accessor<Package[]>,
  doc: Accessor<[number, number, number]>,
  setDoc: Setter<[number, number, number]>
}

export default function Sidebar({ doc, installed, setDoc }: NavProps) {
  const active = "bg-neutral text-neutral-content rounded-md";

  return <>
    <ul class="menu bg-base-200 rounded-box">
      <li class={doc()[0] == -1 ? active : ""} onClick={() => setDoc([-1, 0, 0])}><a>🏠 Home</a></li>
    </ul>
    <ul class="mt-3 menu menu-sm bg-base-200 rounded-box">
      <li>
        <details open>
          <summary><strong>💿 Workspace {!isWorkspace() && <>(not detected)</>}</strong></summary>
          {isWorkspace() && <></>}
        </details>
      </li>
    </ul>
    <ul class="mt-3 menu bg-base-200 rounded-box">
      <li>
        <details open>
          <summary><strong>💾 Installed</strong></summary>
          {
            installed().map((a, pkg) => {
              return <>
                <h2 class="menu-title flex space-x-2 items-center text-center text-base-content"><BsCodeSlash /> <span>{a.name}</span></h2>

                <ul>
                  {a.modules.map((v, mod) => (<li>
                    <details open>
                      <summary class={doc()[0] == pkg && doc()[1] == mod ? active : ""}><a>{v.name}</a></summary>

                      <ul class="mt-1">
                        {v.methods.map((v, fn) => (<li class={doc()[0] == pkg && doc()[1] == mod && doc()[2] == fn ? active : ""} onClick={() => {
                          setDoc([pkg, mod, fn]);
                        }}><a>{v.name}</a></li>))}
                      </ul>
                    </details>
                  </li>))}
                </ul>
              </>
            })
          }
        </details>
      </li>
    </ul>
  </>;
}