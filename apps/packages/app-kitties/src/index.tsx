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
import { SummaryBox, CardSummary } from '@polkadot/react-components';
import Tabs from '@polkadot/react-components/Tabs';
import { ApiPromise } from '@polkadot/api';
// local imports and components
import AccountSelector from './AccountSelector';
import translate from './translate';
import AddKitties from './AddKitties';
// import BreedKitties from './BreedKitties';
// define our internal types
import { ApiContext } from '@polkadot/react-api';
interface Props extends AppProps, I18nProps {}

async function retrieveInfo(api: ApiPromise) {
  try {
    const [kittiesCountQuery] = await Promise.all([
      api.query.kitties.kittiesCount(),
    ]);

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
      <AddKitties />
      {/* <BreedKitties /> */}
    </main>
  );
}

export default translate(App);
