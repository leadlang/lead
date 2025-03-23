import './App.css';
import { Content } from './components/content';
import NavBar from './components/navbar';
import { Sidebar } from './components/sidebar';
import { PageProvider } from './utils/page';

const App = () => {
  return (
    <PageProvider>
      <NavBar />

      <div className="w-full flex gap-2 mb-2">
        <Sidebar />
        <Content />
      </div>
    </PageProvider>
  );
};

export default App;
