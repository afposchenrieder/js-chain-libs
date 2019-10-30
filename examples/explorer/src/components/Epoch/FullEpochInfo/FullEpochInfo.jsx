import React from 'react';

import graphql from 'babel-plugin-relay/macro';
import { createFragmentContainer } from 'react-relay';

import { EmptyResult } from '../../Commons';
import EpochBlockTable from '../EpochBlockTable/EpochBlockTable';
import EpochInfo from '../EpochInfo/EpochInfo';

const FullEpochInfo = ({ epoch }) => {
  if (!epoch) {
    return <EmptyResult {...{ entityName: 'epoch' }} />;
  }

  return (
    <div className="entityInfoContainer">
      <EpochInfo {...{ epoch }} />
      <EpochBlockTable {...{ epoch }} />
    </div>
  );
};

export default createFragmentContainer(FullEpochInfo, {
  epoch: graphql`
    fragment FullEpochInfo_epoch on Epoch {
      ...EpochInfo_epoch
      ...EpochBlockTable_epoch @arguments(last: 10)
    }
  `
});
