import { NETWORK, ACCUDO_API_KEY } from "@/constants";
import { Accudo, AccudoConfig } from "@accudo-labs/ts-sdk";

const accudo = new Accudo(new AccudoConfig({ network: NETWORK, clientConfig: { API_KEY: ACCUDO_API_KEY } }));

// Reuse same Accudo instance to utilize cookie based sticky routing
export function accudoClient() {
  return accudo;
}
