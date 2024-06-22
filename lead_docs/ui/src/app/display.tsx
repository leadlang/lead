import { Accessor } from "solid-js";
import { SolidMarkdown } from "solid-markdown";
import { Package } from "../api/lead";

interface DisplayProps {
  pkg: Accessor<Package[]>,
  doc: Accessor<[number, number, number]>
}

export default function Display({ pkg, doc }: DisplayProps) {
  return <SolidMarkdown>
    {
      doc()[0] != -1 ? pkg()[doc()[0]].modules[doc()[1]].methods[doc()[2]].desc : ""
    }
  </SolidMarkdown>
}