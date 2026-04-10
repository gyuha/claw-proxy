import { useEffect } from 'react';
import { useLogStore } from '../store/logs';
import { connectLogStream } from '../api/ws';

export default function Logs() {
  const { logs, addLog, clear } = useLogStore();

  useEffect(() => {
    const disconnect = connectLogStream(addLog);
    return disconnect;
  }, []);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <h2 className="text-xs font-medium text-zinc-400 uppercase tracking-wider">요청 로그</h2>
        <button onClick={clear} className="text-xs text-zinc-500 hover:text-zinc-300">지우기</button>
      </div>
      <div className="space-y-1">
        {logs.length === 0 && (
          <p className="text-zinc-500 text-xs">요청이 없습니다.</p>
        )}
        {logs.map(log => (
          <div key={log.request_id} className="bg-zinc-800 rounded p-2 text-xs">
            <div className="flex justify-between text-zinc-400">
              <span>{log.model}</span>
              <span className={log.status_code < 400 ? 'text-green-400' : 'text-red-400'}>
                {log.status_code}
              </span>
            </div>
            <div className="flex justify-between text-zinc-500 mt-1">
              <span>{log.provider}</span>
              <span>{log.latency_ms}ms</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
