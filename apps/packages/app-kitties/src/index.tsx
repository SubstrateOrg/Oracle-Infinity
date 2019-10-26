// Copyright 2017-2019 @polkadot/app-123code authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

// some types, AppProps for the app and I18nProps to indicate
// translatable strings. Generally the latter is quite "light",
// `t` is inject into props (see the HOC export) and `t('any text')
// does the translation
import { AppProps, I18nProps } from '@polkadot/react-components/types';

// external imports (including those found in the packages/*
// of this repo)
import React, { useState, useContext, useEffect } from 'react';
import { SummaryBox, CardSummary, TxButton, Input, Button, Column } from '@polkadot/react-components';
import { InputNumber } from '@polkadot/react-components/InputNumber';
import Tabs from '@polkadot/react-components/Tabs';
import { ApiPromise } from '@polkadot/api';
// local imports and components
import AccountSelector from './AccountSelector';
import translate from './translate';
import AddKitties from './AddKitties';
// import BreedKitties from './BreedKitties';
// define our internal types
import { ApiContext } from '@polkadot/react-api';
import BN from 'bn.js';
interface Props extends AppProps, I18nProps {}

async function retrieveInfo(api: ApiPromise) {
  try {
    const [kittiesCountQuery] = await Promise.all([api.query.kitties.kittiesCount()]);
    const kittiesCount = kittiesCountQuery ? kittiesCountQuery.words[0] : 0;

    return { kittiesCount };
  } catch (error) {
    return {};
  }
}

const POLL_TIMEOUT = 9900;

function App({ basePath, t, className }: Props): React.ReactElement<Props> {
  const { api } = useContext(ApiContext);
  const [accountId, setAccountId] = useState<string | null>(null);
  const [kittiesCount, SetKittiesCount] = useState<number>(0);
  const [kittyIndex_offer, setKittyIndex_offer] = useState<string>('0');
  const [offerPrice, setOfferPrice] = useState<BN | undefined>(new BN(0));
  const [kittyIndex_1, setKittyIndex_1] = useState<string>('0');
  const [kittyIndex_2, setKittyIndex_2] = useState<string>('1');
  const [info, setInfo] = useState({});
  const [nextRefresh, setNextRefresh] = useState(Date.now());

  useEffect(() => {
    const _getStatus = (): void => {
      retrieveInfo(api).then(setInfo);
    };

    _getStatus();

    const timerId = window.setInterval((): void => {
      setNextRefresh(Date.now() + POLL_TIMEOUT);
      _getStatus();
    }, POLL_TIMEOUT);

    return (): void => {
      window.clearInterval(timerId);
    };
  }, []);

  return (
    // in all apps, the main wrapper is setup to allow the padding
    // and margins inside the application. (Just from a consistent pov)
    <main className={className}>
      <header>
        <Tabs
          basePath={basePath}
          items={[
            {
              isRoot: true,
              name: 'kitties',
              text: t('Kitties')
            }
          ]}
        />
      </header>
      <SummaryBox>
        <section>
          <CardSummary label={t('Kitties Amount')}>
            <div>{info.kittiesCount}</div>
          </CardSummary>
        </section>
      </SummaryBox>
      <AccountSelector onChange={setAccountId} />
      <Column headerText={t('Add Kitties')}>
        <AddKitties />
      </Column>
      <Column headerText={t('Breed Kitties')}>Breed your two kitties and generate a new one</Column>
      <div className='ui--row'>
        <Input
          autoFocus
          className='medium'
          help={t('Paste here the address of the contact you want to add to your address book.')}
          // isError={!isAddressValid}
          label={t('kitty Index')}
          onChange={setKittyIndex_1}
          value={kittyIndex_1}
        />
        <Input
          autoFocus
          className='medium'
          help={t('Paste here the address of the contact you want to add to your address book.')}
          // isError={!isAddressValid}
          label={t('kitty Index')}
          onChange={setKittyIndex_2}
          value={kittyIndex_2}
        />
      </div>
      <Button.Group>
        <TxButton
          accountId={accountId}
          // isDisabled={!canSubmit}
          isPrimary
          label={t('Breed kitties')}
          icon='sign-in'
          // onClick={onClose}
          params={[kittyIndex_1, kittyIndex_2]}
          tx='kitties.breed'
        />
      </Button.Group>
      <Column headerText={t('Set Price for Kitties')}>Set the price for you kitties</Column>
      <div className='ui--row'>
        <Input
          autoFocus
          className='medium'
          help={t('Paste here the address of the contact you want to add to your address book.')}
          // isError={!isAddressValid}
          label={t('kitty Index')}
          onChange={setKittyIndex_offer}
          value={kittyIndex_offer}
        />
        <Input
          autoFocus
          className='medium'
          help={t('Paste here the address of the contact you want to add to your address book.')}
          // isError={!isAddressValid}
          label={t('Price: balance of')}
          onChange={setOfferPrice}
          value={offerPrice}
        />
      </div>
      <Button.Group>
        <TxButton
          accountId={accountId}
          // isDisabled={!canSubmit}
          isPrimary
          label={t('Set price')}
          icon='sign-in'
          // onClick={onClose}
          params={[kittyIndex_offer, offerPrice]}
          tx='kitties.ask'
        />
      </Button.Group>
      {/* <BreedKitties /> */}
    </main>
  );
}

export default translate(App);
