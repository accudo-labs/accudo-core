import { PropsWithChildren } from "react";
import { AccudoWalletAdapterProvider } from "@accudo-labs/wallet-adapter-react";
// Internal components
import { useToast } from "@/components/ui/use-toast";
// Internal constants
import { ACCUDO_API_KEY, NETWORK } from "@/constants";

export function WalletProvider({ children }: PropsWithChildren) {
  const { toast } = useToast();

  return (
    <AccudoWalletAdapterProvider
      autoConnect={true}
      dappConfig={{ network: NETWORK, accudoApiKeys: {[NETWORK]: ACCUDO_API_KEY} }}
      onError={(error) => {
        toast({
          variant: "destructive",
          title: "Error",
          description: error || "Unknown wallet error",
        });
      }}
    >
      {children}
    </AccudoWalletAdapterProvider>
  );
}
