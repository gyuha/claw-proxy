import AppShell from '../components/shell/AppShell';
import { useRuntimeShellState } from '../features/runtime/state';
import '../styles/app.css';

function App() {
  const runtime = useRuntimeShellState();

  return <AppShell {...runtime} />;
}

export default App;
