import { Accessor } from "solid-js";
import { SolidMarkdown } from "solid-markdown";
import { Package } from "../api/lead";

import gfm from "remark-gfm";
import rehypeRaw from "rehype-raw";
import { version } from "../api/workspace";

interface DisplayProps {
  pkg: Accessor<Package[]>,
  doc: Accessor<[number, number, number]>
}

export default function Display({ pkg, doc }: DisplayProps) {
  const home = `<h1 align="center"><strong>Welcome to Lead Lang Docs</strong></h1>

  This application is call lead lang docs, invoked by the **lead docs** command. This is more/less a markdown renderer but it lists all the methods in an organized way.
  
  # You can currently using **${version()}**`;

  //@ts-ignore
  return <SolidMarkdown class="doc-page" rehypePlugins={[rehypeRaw]} remarkPlugins={[gfm]}>
    {
      doc()[0] != -1 ? pkg()[doc()[0]].modules[doc()[1]].methods[doc()[2]].desc : home
    }
  </SolidMarkdown>
}