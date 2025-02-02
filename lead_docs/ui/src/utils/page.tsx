import { createContext, ReactNode, use, useEffect, useState } from "react";
import { getCore, getWS, Package, Page as PageINTL } from "./const";

export const Page = createContext<PageINTL>({
  r: "home",
  p1: 0,
  p2: "",
  p3: ""
});
export const usePage = () => use(Page);

export const LeadLangRoot = createContext<Package[]>([]);
export const useLeadLang = () => use(LeadLangRoot);

export const WorkspaceRoot = createContext<Package[]>([]);
export const useWorkspace = () => use(WorkspaceRoot);

export function PageProvider({ children }: { children: ReactNode }) {
  const [page, setPage] = useState({
    r: "home",
    p1: 0,
    p2: "",
    p3: ""
  });

  const [rootLL, setLLRoot] = useState<Package[]>([]);
  const [rootWS, setWSRoot] = useState<Package[]>([]);

  useEffect(() => {
    getCore().then(setLLRoot);

    if (window.workspace) {
      getWS().then(setWSRoot);
    }
  }, []);

  window.setPage = setPage;

  return <Page.Provider value={page}>
    <LeadLangRoot.Provider value={rootLL}>
      <WorkspaceRoot.Provider value={rootWS}>
        {children}
      </WorkspaceRoot.Provider>
    </LeadLangRoot.Provider>
  </Page.Provider>
}