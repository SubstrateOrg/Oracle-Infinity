// Copyright 2017-2019 @polkadot/ui-staking authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { Button } from '@polkadot/react-components';
import TxModal, { TxModalState as State, TxModalProps } from '@polkadot/react-components/TxModal';
import { ComponentProps } from './types';
import translate from './translate';

interface Props extends TxModalProps {}

class BreedKitties extends TxModal<Props, State> {
  protected headerText = (): string => this.props.t('Breed kitty');

  protected accountLabel = (): string => this.props.t('Owner account');

  protected accountHelp = (): string => this.props.t('Breed a new kitty under this account.');

  protected txMethod = (): string => 'kitties.breed';

  protected isDisabled = (): boolean => {
    const { accountId } = this.state;

    return !accountId;
  };

  protected renderTrigger = (): React.ReactNode => {
    const { t } = this.props;

    return <Button isPrimary label={t('Breed kitty')} icon='add' onClick={this.showModal} />;
  };
}

export default translate(BreedKitties);
