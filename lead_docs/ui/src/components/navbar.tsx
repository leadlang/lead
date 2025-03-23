import { useEffect, useState } from 'react';
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectItem,
  SelectContent,
} from './ui/select';
import { getTheme, setTheme } from '@/utils/theme';
import { Laptop, Moon, Sun } from 'lucide-react';

export default function NavBar() {
  const [theme, setTTheme] = useState(getTheme());

  useEffect(() => setTheme(theme), [theme]);

  return (
    <div className="h-14 min-h-14 max-h-14 px-3 bg-background/70 flex text-center items-center rounded-md shadow-xl">
      <img src="/icon.png" className="w-6 rounded-sm mr-3 select-none" />
      <h1 className="text-lg select-none">Lead Lang</h1>
      <h3 className="text-sm mb-auto mt-2 ml-1 text-muted-foreground">
        {window.leadver}
      </h3>
      <h3 className="ml-auto">
        {window.os[0].toUpperCase()}
        {window.os.slice(1)} {window.arch} ({window.target})
      </h3>

      <Select
        value={theme}
        onValueChange={(s) => setTTheme(s as 'light' | 'dark' | 'system')}
      >
        <SelectTrigger className="ml-auto w-[150px]">
          <SelectValue placeholder="Theme" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="light">
            <div className="flex items-center gap-1">
              <Sun className="h-4 w-4" />
              <span className="block">Light</span>
            </div>
          </SelectItem>
          <SelectItem value="dark">
            <div className="flex items-center gap-1">
              <Moon className="h-4 w-4" />
              <span className="block">Dark</span>
            </div>
          </SelectItem>
          <SelectItem value="system">
            <div className="flex items-center gap-1">
              <Laptop className="h-4 w-4" />
              <span className="block">System</span>
            </div>
          </SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}
