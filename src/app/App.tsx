import AppShell from '../components/shell/AppShell';
import { useProxyControlState } from '../features/proxy/state';
import { useRuntimeShellState } from '../features/runtime/state';
import '../styles/app.css';

function App() {
  const runtime = useRuntimeShellState();
  const proxy = useProxyControlState();

  return <AppShell {...runtime} proxy={proxy} />;
}

export default App;
