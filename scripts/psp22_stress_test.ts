import { patract, network } from "redspot";
import { LocalKeyringPair } from "redspot/types";
import "@polkadot/api-augment/substrate";

const { getContractFactory } = patract;
const { createSigner, keyring, api, getSigners } = network;

function getChildrenNodes(numChildren: number) {
  const childrenNodes: LocalKeyringPair[] = [];
  for (let i = 0; i < numChildren; ++i) {
    childrenNodes.push(keyring.addFromUri(`//Alice//${i}`));
  }
  return childrenNodes;
}

async function run() {
  await api.isReady;
  const signers = await getSigners();

  const signer = signers[0];
  const signer2 = signers[1];
  const contractFactory = await getContractFactory("psp22", signer);
  const contractFactory2 = await getContractFactory("psp22", signer2);

  const balance = await api.query.system.account(signer.address);
  const balance2 = await api.query.system.account(signer2.address);

  console.log("Balance: ", balance.toHuman());
  console.log("Balance2: ", balance2.toHuman());

  const contract = await contractFactory.deploy("new", "TestToken", "TESTTOKEN", 18, 1_000_000_000);
  const contract2 = await contractFactory2.deploy("new", "TestToken2", "TESTTOKEN2", 18, 1_000_000_000);

  console.log("Deploy successfully. The contract address: ", contract.address.toString());
  console.log("Deploy successfully. The contract2 address: ", contract2.address.toString());

  const totalTx = 200;
  const childrenNodes = getChildrenNodes(totalTx);

  const tasks: any[] = [];
  const tasks2: any[] = [];

  let nonce = (await api.rpc.system.accountNextIndex(signer.address)) as unknown as number;
  let nonce2 = (await api.rpc.system.accountNextIndex(signer2.address)) as unknown as number;

  for (let i = 0; i < totalTx; ++i) {
    const task = contract.tx["psp22::transfer"](childrenNodes[i].address, 1, "0x", { nonce });
    const task2 = contract2.tx["psp22::transfer"](childrenNodes[i].address, 1, "0x", { nonce: nonce2 });
    tasks.push(task);
    tasks2.push(task2);
    ++nonce;
    ++nonce2;
  }

  await Promise.all([...tasks, ...tasks2]);

  api.disconnect();
}

run().catch((err) => {
  console.log(err);
});
