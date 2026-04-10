import type { RuntimeSnapshot } from '../../features/runtime/models';
import RuntimeStatusCard from '../status/RuntimeStatusCard';

interface AppShellProps {
  error: string | null;
  loading: boolean;
  snapshot: RuntimeSnapshot;
}

const futureAreas = [
  {
    name: 'Providers',
    description:
      'Secure provider connections and account pools will appear here without exposing secrets in renderer state.',
  },
  {
    name: 'Routing',
    description:
      'Default model routing, failover strategy, and compatibility controls will build on the runtime-owned snapshot.',
  },
  {
    name: 'Diagnostics',
    description:
      'Health signals, request visibility, and recovery guidance will flow from the host rather than local UI placeholders.',
  },
];

export default function AppShell({
  error,
  loading,
  snapshot,
}: AppShellProps) {
  return (
    <main className="app-shell">
      <div className="app-shell__backdrop" aria-hidden="true" />
      <section className="app-shell__frame">
        <header className="hero-card">
          <div className="hero-card__copy">
            <p className="eyebrow">Local control plane</p>
            <h1>Claw Proxy</h1>
            <p className="hero-card__lede">
              A warm, desktop-first shell for steering the embedded runtime before
              provider security, routing policy, and diagnostics land in later phases.
            </p>
          </div>

          <div className="hero-card__meta">
            <div>
              <span className="hero-card__meta-label">App Ready</span>
              <strong>{snapshot.appReady ? 'Yes' : 'No'}</strong>
            </div>
            <div>
              <span className="hero-card__meta-label">Runtime Mode</span>
              <strong>{snapshot.runtimeStatus}</strong>
            </div>
          </div>
        </header>

        <section className="app-shell__content">
          <RuntimeStatusCard error={error} loading={loading} snapshot={snapshot} />

          <section aria-label="Future product areas" className="future-grid">
            {futureAreas.map((area) => (
              <article key={area.name} className="future-card">
                <p className="eyebrow eyebrow--muted">Upcoming area</p>
                <h2>{area.name}</h2>
                <p>{area.description}</p>
              </article>
            ))}
          </section>
        </section>
      </section>
    </main>
  );
}
