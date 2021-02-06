import React from 'react';
import { Button, Card, Grid, Message, Modal, Form, Label } from 'semantic-ui-react';

import KittyAvatar from './KittyAvatar';
import { TxButton } from './substrate-lib/components';

// --- About Modal ---

const TransferModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);
  const [formValue, setFormValue] = React.useState({});

  const formChange = key => (ev, el) => {
    /* TODO: 加代码 */
    // 修改 key 的值，prev 是引用的先前的值，...prev 表示其余的值保持不变
    setFormValue(prev => ({ ...prev, [key]: el.value }));
  };

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>转让</Button>}>
    <Modal.Header>毛孩转让</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='转让对象' placeholder='对方地址' onChange={formChange('target')}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认转让' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'transfer',
          inputParams: [formValue.target, kitty.id],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
};

const PricingModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);
  const [formValue, setFormValue] = React.useState({});

  const formChange = key => (ev, el) => {
    // 修改 key 的值，prev 是引用的先前的值，...prev 表示其余的值保持不变
    setFormValue(prev => ({ ...prev, [key]: el.value }));
  };

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>定价</Button>}>
    <Modal.Header>毛孩定价</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='新价格' placeholder='售卖价格' onChange={formChange('price')}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认更改' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'ask',
          inputParams: [kitty.id, formValue.price],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
};

const BuyModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>购买</Button>}>
    <Modal.Header>毛孩购买让</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='价格' readOnly value={kitty.price}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认购买' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'buy',
          inputParams: [kitty.id, kitty.price],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
};

// --- About Kitty Card ---

const KittyCard = props => {
  /*
    TODO: 加代码。这里会 UI 显示一张 `KittyCard` 是怎么样的。这里会用到：
    ```
    <KittyAvatar dna={dna} /> - 来描绘一只猫咪
    <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/> - 来作转让的弹出层
    ```
  */
  const { kitty, owner, price, accountPair, setStatus } = props;

  if (owner === accountPair.address) {
    kitty.is_owner = true;
  }
  kitty.price = price;

  return (
    <Grid.Column>
      <Card style={{ wordBreak: 'break-all', width: '275px', margin: '5px' }}>
        <Card.Content>
          <Card.Header>ID {kitty.id}</Card.Header>
          <KittyAvatar dna={kitty.dna} />
          <Card.Description>
            <b>dna: {kitty.dna}</b> <br />
            <b>price: {kitty.price}</b> <br />
            <b>owner: {kitty.is_owner ? 'mine' : owner }</b> <br />
          </Card.Description>
          {
            kitty.is_owner && <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
          }
          {
            kitty.is_owner && <PricingModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
          }
          {
            !kitty.is_owner && <BuyModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
          }
        </Card.Content>
      </Card>
    </Grid.Column>
  );
};

const KittyCards = props => {
  const { kitties, kittyOwners, kittyPrices, accountPair, setStatus } = props;

  /* TODO: 加代码。这里会枚举所有的 `KittyCard` */
  return (
    <Grid stackable columns='equal'>
      <Grid.Row stretched>
        {
          kitties.map((kitty, index) => {
            return (
              <Grid.Row key={index}>
                <KittyCard
                  kitty={kitty}
                  owner={kittyOwners[index]}
                  price={kittyPrices[index]}
                  accountPair={accountPair}
                  setStatus={setStatus}
                />
              </Grid.Row>
            );
          })
        }
      </Grid.Row>
    </Grid>
  );
};

export default KittyCards;
