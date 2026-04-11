import {
  startTransition,
  useEffect,
  useEffectEvent,
  useState,
} from 'react';

import {
  applyProxySettings as applyProxySettingsFromHost,
  getProxyRuntimeSnapshot,
  getProxySettings,
  startProxyRuntime as startProxyRuntimeFromHost,
  stopProxyRuntime as stopProxyRuntimeFromHost,
  subscribeToProxyRuntimeUpdates,
} from './api';
import {
  createInitialProxyRuntimeSnapshot,
  createInitialProxySettings,
  loopbackHostOptions,
  type ProxyListenHost,
  type ProxyRuntimeSnapshot,
  type ProxySettings,
} from './models';

export interface ProxyControlState {
  applySettings: () => Promise<void>;
  draft: ProxySettings;
  error: string | null;
  hostOptions: readonly ProxyListenHost[];
  isApplying: boolean;
  loading: boolean;
  refresh: () => Promise<void>;
  snapshot: ProxyRuntimeSnapshot;
  startProxy: () => Promise<void>;
  stopProxy: () => Promise<void>;
  updateDraft: <Key extends keyof ProxySettings>(
    field: Key,
    value: ProxySettings[Key],
  ) => void;
}

function areProxySettingsEqual(left: ProxySettings, right: ProxySettings): boolean {
  return (
    left.baseEndpoint === right.baseEndpoint &&
    left.listenHost === right.listenHost &&
    left.listenPort === right.listenPort
  );
}

function toErrorMessage(error: unknown): string {
  if (
    typeof error === 'object' &&
    error !== null &&
    'message' in error &&
    typeof error.message === 'string'
  ) {
    return error.message;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return 'Unable to update the local proxy.';
}

export function useProxyControlState(): ProxyControlState {
  const [snapshot, setSnapshot] = useState<ProxyRuntimeSnapshot>(() =>
    createInitialProxyRuntimeSnapshot(),
  );
  const [canonicalSettings, setCanonicalSettings] = useState<ProxySettings>(() =>
    createInitialProxySettings(),
  );
  const [draft, setDraft] = useState<ProxySettings>(() =>
    createInitialProxySettings(),
  );
  const [loading, setLoading] = useState(true);
  const [isApplying, setIsApplying] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const syncFromHost = useEffectEvent(
    async ({
      preserveDraft = false,
      preserveError = false,
    }: {
      preserveDraft?: boolean;
      preserveError?: boolean;
    } = {}) => {
      setLoading(true);

      if (!preserveError) {
        setError(null);
      }

      try {
        const [nextSnapshot, nextSettings] = await Promise.all([
          getProxyRuntimeSnapshot(),
          getProxySettings(),
        ]);

        startTransition(() => {
          setSnapshot(nextSnapshot);
          setCanonicalSettings(nextSettings);

          if (!preserveDraft || areProxySettingsEqual(draft, canonicalSettings)) {
            setDraft(nextSettings);
          }
        });
      } catch (caughtError) {
        startTransition(() => {
          setError(toErrorMessage(caughtError));
        });
      } finally {
        setLoading(false);
      }
    },
  );

  useEffect(() => {
    let isActive = true;
    let disposeUpdates: (() => void) | undefined;

    async function initializeProxyState() {
      await syncFromHost();

      if (!isActive) {
        return;
      }

      disposeUpdates = await subscribeToProxyRuntimeUpdates(async () => {
        if (!isActive) {
          return;
        }

        await syncFromHost({
          preserveDraft: true,
          preserveError: true,
        });
      });
    }

    void initializeProxyState();

    return () => {
      isActive = false;
      disposeUpdates?.();
    };
  }, []);

  const refresh = async () => {
    await syncFromHost({
      preserveDraft: true,
      preserveError: true,
    });
  };

  const updateDraft = <Key extends keyof ProxySettings>(
    field: Key,
    value: ProxySettings[Key],
  ) => {
    setDraft((currentDraft) => ({
      ...currentDraft,
      [field]: value,
    }));
  };

  const startProxy = async () => {
    setIsApplying(true);
    setError(null);

    try {
      await startProxyRuntimeFromHost();
      await syncFromHost({
        preserveDraft: true,
      });
    } catch (caughtError) {
      setError(toErrorMessage(caughtError));
      await syncFromHost({
        preserveDraft: true,
        preserveError: true,
      });
    } finally {
      setIsApplying(false);
    }
  };

  const stopProxy = async () => {
    setIsApplying(true);
    setError(null);

    try {
      await stopProxyRuntimeFromHost();
      await syncFromHost({
        preserveDraft: true,
      });
    } catch (caughtError) {
      setError(toErrorMessage(caughtError));
      await syncFromHost({
        preserveDraft: true,
        preserveError: true,
      });
    } finally {
      setIsApplying(false);
    }
  };

  const applySettings = async () => {
    setIsApplying(true);
    setError(null);

    try {
      await applyProxySettingsFromHost(draft);
      await syncFromHost();
    } catch (caughtError) {
      setError(toErrorMessage(caughtError));
      await syncFromHost({
        preserveDraft: true,
        preserveError: true,
      });
    } finally {
      setIsApplying(false);
    }
  };

  return {
    applySettings,
    draft,
    error,
    hostOptions: loopbackHostOptions,
    isApplying,
    loading,
    refresh,
    snapshot,
    startProxy,
    stopProxy,
    updateDraft,
  };
}
