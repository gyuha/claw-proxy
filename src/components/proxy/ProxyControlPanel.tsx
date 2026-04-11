import type { ProxyRuntimeSnapshot } from '../../features/proxy/models';

interface ProxyControlPanelProps {
  busy: boolean;
  error: string | null;
  onStart: () => Promise<void>;
  onStop: () => Promise<void>;
  snapshot: ProxyRuntimeSnapshot;
}

const proxyStatusContent = {
  healthy: {
    badge: 'Healthy',
    description:
      'The embedded proxy is serving the current local endpoint and ready for connected tools.',
  },
  misconfigured: {
    badge: 'Misconfigured',
    description:
      'The host kept the last known good configuration. Fix the draft settings, then apply again.',
  },
  starting: {
    badge: 'Starting',
    description:
      'Claw Proxy is restarting the local proxy and waiting for the host-owned health check to settle.',
  },
  stopped: {
    badge: 'Stopped',
    description:
      'The proxy is idle. Start it from here when you want the local endpoint online.',
  },
} as const;

function formatTransition(value: string | null): string {
  if (!value) {
    return 'Not recorded yet';
  }

  const timestamp = Number(value) * 1000;
  if (Number.isNaN(timestamp)) {
    return value;
  }

  return new Date(timestamp).toLocaleString();
}

export default function ProxyControlPanel({
  busy,
  error,
  onStart,
  onStop,
  snapshot,
}: ProxyControlPanelProps) {
  const content = proxyStatusContent[snapshot.status];
  const effectiveError = error ?? snapshot.lastError;

  return (
    <section aria-label="Proxy control surface" className="proxy-card">
      <div className="proxy-card__header">
        <div>
          <p className="eyebrow">Proxy Control</p>
          <h2>Local proxy health</h2>
        </div>
        <span className={`status-pill status-pill--${snapshot.status}`}>
          {content.badge}
        </span>
      </div>

      <p className="proxy-card__description">
        {effectiveError && snapshot.status === 'misconfigured'
          ? effectiveError
          : content.description}
      </p>

      <dl className="proxy-card__stats">
        <div>
          <dt>Active Local URL</dt>
          <dd>{snapshot.effectiveBaseUrl}</dd>
        </div>
        <div>
          <dt>Listen Host</dt>
          <dd>{snapshot.settings.listenHost}</dd>
        </div>
        <div>
          <dt>Base Endpoint</dt>
          <dd>{snapshot.settings.baseEndpoint}</dd>
        </div>
        <div>
          <dt>Last Transition</dt>
          <dd>{formatTransition(snapshot.lastTransitionAt)}</dd>
        </div>
      </dl>

      <div className="proxy-card__actions">
        <button
          className="proxy-button proxy-button--primary"
          disabled={busy || snapshot.status === 'healthy' || snapshot.status === 'starting'}
          onClick={() => {
            void onStart();
          }}
          type="button"
        >
          Start Proxy
        </button>
        <button
          className="proxy-button proxy-button--secondary"
          disabled={busy || snapshot.status === 'stopped'}
          onClick={() => {
            void onStop();
          }}
          type="button"
        >
          Stop Proxy
        </button>
      </div>
    </section>
  );
}
