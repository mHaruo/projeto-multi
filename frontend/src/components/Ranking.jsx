import React, { useEffect, useState } from 'react';
import { useGetNetworkConfig } from '@multiversx/sdk-dapp/hooks';
import { ProxyNetworkProvider } from '@multiversx/sdk-network-providers';
import { Address } from '@multiversx/sdk-core';

const contractAddress = 'SEU_CONTRACT_ADDRESS';

export default function Ranking() {
  const [users, setUsers] = useState([]);
  const { network } = useGetNetworkConfig();

  useEffect(() => {
    const fetchRanking = async () => {
      const provider = new ProxyNetworkProvider(network.apiAddress);
      const contract = new Address(contractAddress);

      try {
        const { returnData } = await provider.queryContract(contract, {
          funcName: 'get_all_users', // ajuste conforme o SC
        });

        const decodedUsers = returnData.map(data => {
          const [nameHex, starsHex] = data.values;
          return {
            name: Buffer.from(nameHex, 'hex').toString(),
            stars: parseInt(starsHex, 16),
          };
        });

        setUsers(decodedUsers);
      } catch (error) {
        console.error('Erro ao buscar ranking:', error);
      }
    };

    fetchRanking();
  }, [network.apiAddress]);

  const getMedal = (stars) => {
    if (stars >= 5) return '🥇 Ouro (3 votos)';
    if (stars >= 2) return '🥈 Prata (2 votos)';
    return '🥉 Bronze (1 voto)';
  };

  return (
    <div>
      <h2>Ranking de Reputação ✨</h2>
      <ul>
        {users.sort((a, b) => b.stars - a.stars).map((user, index) => (
          <li key={index}>
            <strong>{user.name}</strong> — ⭐ {user.stars} — {getMedal(user.stars)}
          </li>
        ))}
      </ul>
    </div>
  );
}
