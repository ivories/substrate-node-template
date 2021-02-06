import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

export default function Kitties (props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kittyCnt, setKittyCnt] = useState(0);
  const [kittyOwners, setKittyOwners] = useState([]);
  const [kittyPrices, setKittyPrices] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');

  const fetchKittyCnt = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    let unsubscribe;
    api.query.kittiesModule.kittiesCount(count => {
      setKittyCnt(count.toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  const fetchKitties = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    if (kittyCnt === 0) return;

    let unsubscribe;
    const ids = [...Array(kittyCnt).keys()];
    api.query.kittiesModule.kitties.multi(ids, kitties => {
      kitties = kitties.map((item, index) => ({ id: index, dna: item.unwrap().toString(), is_owner: false, price: '0' }));
      setKitties(kitties);
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  const fetchKittyOwners = () => {
    if (kittyCnt === 0) return;

    let unsubscribe;
    const ids = [...Array(kittyCnt).keys()];
    api.query.kittiesModule.kittyOwners.multi(ids, owners => {
      owners = owners.map(item => (item.unwrap().toString()));
      setKittyOwners(owners);
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  const fetchKittyPrices = () => {
    if (kittyCnt === 0) return;

    let unsubscribe;
    const ids = [...Array(kittyCnt).keys()];
    api.query.kittiesModule.kittyPrices.multi(ids, prices => {
      prices = prices.map(item => (item.isNone ? '0' : item.unwrap().toString()));
      setKittyPrices(prices);
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  useEffect(fetchKittyCnt, [api, keyring]);
  useEffect(fetchKitties, [api, kittyCnt, kittyOwners, accountPair]);
  useEffect(fetchKittyOwners, [api, kittyCnt, accountPair]);
  useEffect(fetchKittyPrices, [api, kittyCnt, accountPair]);

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <h3>共 {kittyCnt} 小毛孩</h3>
    <KittyCards kitties={kitties} kittyOwners={kittyOwners} kittyPrices={kittyPrices} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>;
}
