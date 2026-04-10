import { create } from 'zustand';
import { getProviders } from '../api/client';
import type { ProviderInfo } from '../types';

interface ProviderStore {
  providers: ProviderInfo[];
  fetch: () => Promise<void>;
}

export const useProviderStore = create<ProviderStore>((set) => ({
  providers: [],
  fetch: async () => {
    try {
      const providers = await getProviders();
      set({ providers });
    } catch {}
  },
}));
