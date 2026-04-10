import type { RuntimeSnapshot } from '../../features/runtime/models';

interface RuntimeStatusCardProps {
  error: string | null;
  loading: boolean;
  snapshot: RuntimeSnapshot;
}

const statusContent = {
  error: {
    badge: 'Needs Attention',
    description: 'The renderer can talk to the host, but the runtime reported an error state.',
  },
  ready: {
    badge: 'Healthy',
    description: 'The embedded runtime is initialized and ready to back the control surface.',
  },
  starting: {
    badge: 'Syncing',
    description: 'Claw Proxy is initializing the embedded runtime and collecting its first snapshot.',
  },
  stopped: {
    badge: 'Stopped',
    description: 'The runtime is reachable but not actively serving the control-plane workflow yet.',
  },
} as const;

function formatHealthCheck(value: string | null): string {
  if (!value) {
    return 'Not recorded yet';
  }

  const timestamp = Number(value) * 1000;

  if (Number.isNaN(timestamp)) {
    return value;
  }

  return new Date(timestamp).toLocaleString();
}

export default function RuntimeStatusCard({
  error,
  loading,
  snapshot,
}: RuntimeStatusCardProps) {
  const effectiveStatus = error ? 'error' : snapshot.runtimeStatus;
  const content = statusContent[effectiveStatus];

  return (
    <section aria-label="Runtime status" className="runtime-card">
      <div className="runtime-card__header">
        <div>
          <p className="eyebrow">Runtime Status</p>
          <h2>{loading ? 'Connecting to Claw Proxy' : 'Runtime authority online'}</h2>
        </div>
        <span className={`status-pill status-pill--${effectiveStatus}`}>
          {loading ? 'Loading' : content.badge}
        </span>
      </div>

      <p className="runtime-card__description">
        {error ?? content.description}
      </p>

      <dl className="runtime-card__stats">
        <div>
          <dt>Profile</dt>
          <dd>{snapshot.activeProfile ?? 'No profile selected'}</dd>
        </div>
        <div>
          <dt>Providers</dt>
          <dd>{snapshot.providerSlots}</dd>
        </div>
        <div>
          <dt>Accounts</dt>
          <dd>{snapshot.accountSlots}</dd>
        </div>
        <div>
          <dt>Last Health Check</dt>
          <dd>{formatHealthCheck(snapshot.lastHealthCheck)}</dd>
        </div>
      </dl>
    </section>
  );
}
