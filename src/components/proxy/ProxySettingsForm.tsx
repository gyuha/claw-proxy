import {
  buildProxyEffectiveBaseUrl,
  type ProxyListenHost,
  type ProxySettings,
} from '../../features/proxy/models';

interface ProxySettingsFormProps {
  busy: boolean;
  draft: ProxySettings;
  hostOptions: readonly ProxyListenHost[];
  onApply: () => Promise<void>;
  onDraftChange: <Key extends keyof ProxySettings>(
    field: Key,
    value: ProxySettings[Key],
  ) => void;
}

export default function ProxySettingsForm({
  busy,
  draft,
  hostOptions,
  onApply,
  onDraftChange,
}: ProxySettingsFormProps) {
  return (
    <section aria-label="Proxy settings" className="proxy-form">
      <div className="proxy-form__header">
        <div>
          <p className="eyebrow">Proxy Settings</p>
          <h2>Loopback-only configuration</h2>
        </div>
        <p className="proxy-form__note">
          Phase 2 supports local loopback hosts only.
        </p>
      </div>

      <div className="proxy-form__fields">
        <label className="proxy-field">
          <span>Listen Host</span>
          <select
            disabled={busy}
            onChange={(event) => {
              onDraftChange('listenHost', event.target.value as ProxyListenHost);
            }}
            value={draft.listenHost}
          >
            {hostOptions.map((host) => (
              <option key={host} value={host}>
                {host}
              </option>
            ))}
          </select>
        </label>

        <label className="proxy-field">
          <span>Listen Port</span>
          <input
            disabled={busy}
            min={1}
            onChange={(event) => {
              onDraftChange('listenPort', Number(event.target.value));
            }}
            type="number"
            value={draft.listenPort}
          />
        </label>

        <label className="proxy-field">
          <span>Base Endpoint</span>
          <input
            disabled={busy}
            onChange={(event) => {
              onDraftChange('baseEndpoint', event.target.value);
            }}
            type="text"
            value={draft.baseEndpoint}
          />
        </label>
      </div>

      <div className="proxy-preview">
        <span className="proxy-preview__label">Effective Local URL</span>
        <strong>{buildProxyEffectiveBaseUrl(draft)}</strong>
      </div>

      <button
        className="proxy-button proxy-button--primary"
        disabled={busy}
        onClick={() => {
          void onApply();
        }}
        type="button"
      >
        Apply Settings
      </button>
    </section>
  );
}
