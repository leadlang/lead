export default function root() {
  if (window.os == 'windows') {
    return 'http://api.localhost';
  } else {
    return 'api://localhost';
  }
}

export interface Page {
  r: string;
  p1: number;
  p2: string;
  p3: string;
  rt: boolean;
}

export interface Package {
  name: string;
  doc: {
    [key: string]: {
      [key: string]: string;
    };
  };
  runtimes: {
    [key: string]: [
      string,
      {
        [key: string]: string;
      },
    ];
  };
}

export async function getCore(): Promise<Package[]> {
  const response = await fetch(`${root()}/core`);
  const data = await response.json();
  return data;
}

export async function getWS(): Promise<Package[]> {
  const response = await fetch(`${root()}/workspace`);
  const data = await response.json();
  return data;
}
