// Copyright 2017-2019 @polkadot/ui-staking authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { ComponentProps } from './types';

import BN from 'bn.js';
import React from 'react';
import { Button } from '@polkadot/react-components';
import TxModal, { TxModalState as State, TxModalProps } from '@polkadot/react-components/TxModal';

import translate from './translate';

interface Props extends TxModalProps {}

class AddKitties extends TxModal<Props, State> {
  protected headerText = (): string => this.props.t('Craete your kitty');

  protected accountLabel = (): string => this.props.t('Owner account');

  protected accountHelp = (): string => this.props.t('This account will be owned the kitty that generated.');

  protected txMethod = (): string => 'kitties.create';

  protected isDisabled = (): boolean => {
    const { accountId } = this.state;

    return !accountId;
  };

  protected renderTrigger = (): React.ReactNode => {
    const { t } = this.props;

    return <Button isPrimary label={t('Create kitty')} icon='add' onClick={this.showModal} />;
  };
}

export default translate(AddKitties);
