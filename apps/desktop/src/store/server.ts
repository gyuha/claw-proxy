import { create } from 'zustand';
import { getStatus } from '../api/client';
import type { ServerStatus } from '../types';

interface ServerStore {
  status: ServerStatus | null;
  loading: boolean;
  fetch: () => Promise<void>;
}

export const useServerStore = create<ServerStore>((set) => ({
  status: null,
  loading: false,
  fetch: async () => {
    set({ loading: true });
    try {
      const status = await getStatus();
      set({ status, loading: false });
    } catch {
      set({ loading: false });
    }
  },
}));
