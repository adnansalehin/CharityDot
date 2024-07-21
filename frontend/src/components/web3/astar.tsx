import React, { useEffect, useState } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { web3FromAddress } from '@polkadot/extension-dapp';
// import contractAbi from './path/to/contract_abi.json';

const CONTRACT_ADDRESS = 'your_contract_address';
const WS_PROVIDER = 'wss://astar-node-url';

const App = () => {
  const [api, setApi] = useState(null);
  const [account, setAccount] = useState(null);
  const [contract, setContract] = useState(null);

  useEffect(() => {
    const connectToNode = async () => {
      const provider = new WsProvider(WS_PROVIDER);
      const api = await ApiPromise.create({ provider });
    //   setApi(api);
    };

    connectToNode();
  }, []);

  useEffect(() => {
    if (api) {
      const contract = new ContractPromise(api, contractAbi, CONTRACT_ADDRESS);
    //   setContract(contract);
    }
  }, [api]);

  const donate = async (charityAccount, amount) => {
    if (!account) {
      console.error('No account selected');
      return;
    }

    const injector = await web3FromAddress(account.address);
    await contract.tx.donate({ value: amount }, charityAccount).signAndSend(account.address, { signer: injector.signer });
  };

  return (
    <div>
      <h1>Astar Donation DApp</h1>
      <button onClick={() => donate('charity_account_id', 1000)}>Donate</button>
    </div>
  );
};

export default App;