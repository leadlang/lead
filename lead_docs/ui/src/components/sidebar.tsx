import { Package } from "@/utils/const";
import { useLeadLang, usePage } from "@/utils/page";

export function Sidebar() {
  const page = usePage();
  const leadCore = useLeadLang();

  return <div className="bg-background/70 h-limits flex justify-center rounded-md shadow-xl overflow-hidden">
    <ul className="menu flex-col w-80 flex-nowrap bg-inherit/70 text-foreground gap-1 overflow-scroll">
      <li><button className={page.r == "home" ? "active" : ""} onClick={() => window.setPage({
        r: "home",
        p1: 0,
        p2: "",
        p3: ""
      })}>🏠 Home</button></li>
      {/* Core */}
      <PackageArray prefix="lead" data={leadCore} summary="📚 Core Libraries" />

      {
        window.workspace
          ? <PackageArray prefix="ws" data={leadCore} summary="⚒️ Workspace" />
          : <li><button disabled className="cursor-default">⚒️ Workspace (Not Found)</button></li>
      }
    </ul>
  </div>;
}

interface Props {
  prefix: string;
  summary: string;
  data: Package[]
}

function PackageArray({ data, prefix, summary }: Props) {
  const page = usePage();

  return <li>
    <details open>
      <summary className={addApply(page.r == prefix)}>{summary}</summary>

      <ul>
        {
          data.sort((a, b) => a.name.localeCompare(b.name)).map((item, item_index) => (
            <li key={item.name}>
              <details open>
                <summary className={addApply(page.r == prefix && page.p1 == item_index)}>{item.name}</summary>

                <ul>
                  {
                    Object.entries(item.doc)
                      .sort(([a,], [b,]) => a.localeCompare(b))
                      .map(([name, val]) => (
                        <li key={`${item.name}${name}`} >
                          <details>
                            <summary className={addApply(page.r == prefix && page.p1 == item_index && page.p2 == name)}>{name}</summary>

                            <ul>
                              {
                                Object.entries(val)
                                  .sort(([a,], [b,]) => a.localeCompare(b))
                                  .map(([n,]) => (
                                    <li key={`${item.name}${name}${n}`}>
                                      <button
                                        className={addApply(page.r == prefix && page.p1 == item_index && page.p2 == name && page.p3 == n)}
                                        onClick={() => window.setPage({
                                          r: prefix,
                                          p1: item_index,
                                          p2: name,
                                          p3: n
                                        })}
                                      >{n}</button>
                                    </li>
                                  ))
                              }
                            </ul>
                          </details>
                        </li>
                      ))
                  }
                </ul>
              </details>
            </li>
          ))
        }
      </ul>
    </details>
  </li>
}

const addApply = (cond: boolean) => cond ? "active" : "";