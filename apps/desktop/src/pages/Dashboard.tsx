import { useEffect } from 'react';
import { useServerStore } from '../store/server';
import { useProviderStore } from '../store/providers';

export default function Dashboard() {
  const { status, fetch: fetchStatus } = useServerStore();
  const { providers, fetch: fetchProviders } = useProviderStore();

  useEffect(() => {
    fetchStatus();
    fetchProviders();
    const interval = setInterval(fetchStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const activeCount = providers.filter(p => p.available).length;

  return (
    <div className="space-y-4">
      <div className="bg-zinc-800 rounded-lg p-4">
        <div className="flex items-center justify-between">
          <span className="text-zinc-300">서버 상태</span>
          {status ? (
            <span className="text-green-400 text-xs">● 실행 중</span>
          ) : (
            <span className="text-red-400 text-xs">● 연결 중...</span>
          )}
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="bg-zinc-800 rounded-lg p-3">
          <div className="text-zinc-500 text-xs">활성 프로바이더</div>
          <div className="text-2xl font-bold mt-1">{activeCount}</div>
        </div>
        <div className="bg-zinc-800 rounded-lg p-3">
          <div className="text-zinc-500 text-xs">업타임</div>
          <div className="text-2xl font-bold mt-1">
            {status ? `${Math.floor(status.uptime_secs / 60)}m` : '--'}
          </div>
        </div>
      </div>
    </div>
  );
}
