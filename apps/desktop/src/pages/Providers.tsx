import { useEffect } from 'react';
import { useProviderStore } from '../store/providers';

export default function Providers() {
  const { providers, fetch } = useProviderStore();

  useEffect(() => { fetch(); }, []);

  return (
    <div className="space-y-2">
      <h2 className="text-xs font-medium text-zinc-400 uppercase tracking-wider">프로바이더</h2>
      {providers.length === 0 && (
        <p className="text-zinc-500 text-xs">등록된 프로바이더가 없습니다.</p>
      )}
      {providers.map(p => (
        <div key={p.name} className="bg-zinc-800 rounded-lg p-3 flex items-center justify-between">
          <span className="text-zinc-200 text-xs">{p.name}</span>
          <span className={`text-xs ${p.available ? 'text-green-400' : 'text-red-400'}`}>
            {p.available ? '● 활성' : '● 비활성'}
          </span>
        </div>
      ))}
    </div>
  );
}
