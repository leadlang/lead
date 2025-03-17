import { useLeadLang, usePage, useWorkspace } from '@/utils/page';
import Markdown from 'react-markdown'

import remarkGfm from 'remark-gfm'

import raw from "rehype-raw"

export function Content() {
  const markdown = useMarkdown();

  return <div className='w-full h-full bg-background/70 rounded-md markdown flex flex-col p-6 h-limits overflow-scroll'>
    <Markdown remarkPlugins={[remarkGfm]} rehypePlugins={[raw]}>{markdown}</Markdown>
  </div>;
}

function useMarkdown(): string {
  const {
    p1,
    p2,
    p3,
    r,
    rt
  } = usePage();

  const lead = useLeadLang();
  const ws = useWorkspace();

  switch (r) {
    case "home":
      return `<div class="h-full flex flex-col items-center">
<img src="/icon.png" class="rounded-md" style="height:5rem;width:5rem;" />
<h1 style="text-align:center;">The Lead Programming Language Documentation</h1>
<h4 class="text-muted-foreground" style="text-align:center;">Documentation like never before!</h4>

<div class="flex w-full text-center items-center justify-center gap-2">
  <a href="https://leadlang.github.io" target="_blank" class="but">Website</a>
  <a href="https://github.com/leadlang" target="_blank" class="but">GitHub</a>
</div>

<h5 class="text-muted-foreground" style="text-align:center;margin-top:auto;">Rendered using markdown!</h5></div>
      `;
    case "lead":
      if (rt) {
        return lead[p1].runtimes[p2][1][p3];
      }
      else {
        return lead[p1].doc[p2][p3];
      }
    case "ws":
      if (rt) {
        return ws[p1].runtimes[p2][1][p3];
      }
      else {
        return ws[p1].doc[p2][p3];
      }
    default:
      return `# Not Found`
  }
}