require("dotenv").config();
const cli = require("@accudo-labs/ts-sdk/dist/common/cli/index.js");
const accudoSDK = require("@accudo-labs/ts-sdk")

async function publish() {
  if (!process.env.VITE_MODULE_ADDRESS) {
    throw new Error(
      "VITE_MODULE_ADDRESS variable is not set, make sure you have published the module before upgrading it",
    );
  }

  const move = new cli.Move();

  move.upgradeObjectPackage({
    packageDirectoryPath: "contract",
    objectAddress: process.env.VITE_MODULE_ADDRESS,
    namedAddresses: {
      // Upgrade module from an object
      todolist_addr: process.env.VITE_MODULE_PUBLISHER_ACCOUNT_ADDRESS,
    },
    extraArguments: [`--private-key=${process.env.VITE_MODULE_PUBLISHER_ACCOUNT_PRIVATE_KEY}`,`--url=${accudoSDK.NetworkToNodeAPI[process.env.VITE_APP_NETWORK]}`],
  });
}
publish();
