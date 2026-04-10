import type { LogEntry } from '../../../packages/shared-types/src';

const WS_URL = 'ws://localhost:47381/ws';

export function connectLogStream(onEntry: (entry: LogEntry) => void): () => void {
  const ws = new WebSocket(WS_URL);

  ws.onmessage = (event) => {
    try {
      const entry: LogEntry = JSON.parse(event.data);
      onEntry(entry);
    } catch {}
  };

  return () => ws.close();
}
