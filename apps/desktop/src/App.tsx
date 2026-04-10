import { useState } from 'react';
import Dashboard from './pages/Dashboard';
import Providers from './pages/Providers';
import Logs from './pages/Logs';
import Settings from './pages/Settings';

type Page = 'dashboard' | 'providers' | 'logs' | 'settings';

export default function App() {
  const [page, setPage] = useState<Page>('dashboard');

  const nav = [
    { id: 'dashboard' as Page, label: '대시보드' },
    { id: 'providers' as Page, label: '프로바이더' },
    { id: 'logs' as Page, label: '로그' },
    { id: 'settings' as Page, label: '설정' },
  ];

  return (
    <div className="flex flex-col h-screen bg-zinc-900 text-white text-sm">
      {/* 상태 헤더 */}
      <div className="flex items-center justify-between px-4 py-2 bg-zinc-800 border-b border-zinc-700">
        <span className="font-medium">🦀 Claw Proxy</span>
        <span className="text-xs text-green-400">● 실행 중 :47380</span>
      </div>

      <div className="flex flex-1 overflow-hidden">
        {/* 사이드 네비게이션 */}
        <nav className="w-24 bg-zinc-800 border-r border-zinc-700 py-2">
          {nav.map(item => (
            <button
              key={item.id}
              onClick={() => setPage(item.id)}
              className={`w-full px-2 py-3 text-xs text-center hover:bg-zinc-700 transition-colors ${
                page === item.id ? 'bg-zinc-700 text-white' : 'text-zinc-400'
              }`}
            >
              {item.label}
            </button>
          ))}
        </nav>

        {/* 메인 콘텐츠 */}
        <main className="flex-1 overflow-y-auto p-4">
          {page === 'dashboard' && <Dashboard />}
          {page === 'providers' && <Providers />}
          {page === 'logs' && <Logs />}
          {page === 'settings' && <Settings />}
        </main>
      </div>
    </div>
  );
}
