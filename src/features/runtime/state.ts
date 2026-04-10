import { useEffect, useState } from 'react';

import {
  getRuntimeSnapshot,
  initializeRuntimeState,
  type RuntimePingResponse,
} from './api';
import { createInitialRuntimeSnapshot, type RuntimeSnapshot } from './models';

interface RuntimeShellState {
  error: string | null;
  loading: boolean;
  ping: RuntimePingResponse | null;
  refresh: () => Promise<void>;
  snapshot: RuntimeSnapshot;
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }

  return 'Unable to reach the embedded runtime.';
}

export function useRuntimeShellState(): RuntimeShellState {
  const [snapshot, setSnapshot] = useState<RuntimeSnapshot>(() =>
    createInitialRuntimeSnapshot(),
  );
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [ping, setPing] = useState<RuntimePingResponse | null>(null);

  const refresh = async () => {
    setLoading(true);
    setError(null);

    try {
      const nextSnapshot = await getRuntimeSnapshot();
      setSnapshot(nextSnapshot);
    } catch (caughtError) {
      setError(toErrorMessage(caughtError));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    let isActive = true;

    async function initializeShell() {
      setLoading(true);
      setError(null);

      try {
        const nextSnapshot = await initializeRuntimeState();

        if (isActive) {
          setSnapshot(nextSnapshot);
        }
      } catch (caughtError) {
        if (isActive) {
          setError(toErrorMessage(caughtError));
        }
      } finally {
        if (isActive) {
          setLoading(false);
        }
      }
    }

    void initializeShell();

    return () => {
      isActive = false;
    };
  }, []);

  return {
    error,
    loading,
    ping,
    refresh,
    snapshot,
  };
}
