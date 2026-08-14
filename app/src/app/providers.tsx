'use client';

import React, { useMemo } from 'react';
import { SolanaProvider } from '@solana/react-hooks';
import { createClient, autoDiscover } from '@solana/client';
import { SelectedWalletAccountContextProvider } from '@solana/react';

const RPC_ENDPOINT = process.env.NEXT_PUBLIC_SOLANA_RPC_URL ?? 'https://api.devnet.solana.com';

export function Providers({ children }: { children: React.ReactNode }) {
  const solanaClient = useMemo(() => createClient({
    endpoint: RPC_ENDPOINT,
    websocketEndpoint: RPC_ENDPOINT.replace('https://', 'wss://').replace('http://', 'ws://'),
    walletConnectors: autoDiscover(),
  }), []);

  return (
    <SolanaProvider client={solanaClient}>
      <SelectedWalletAccountContextProvider
        filterWallet={(wallet) => wallet.accounts.length > 0}
      >
        {children}
      </SelectedWalletAccountContextProvider>
    </SolanaProvider>
  );
}
