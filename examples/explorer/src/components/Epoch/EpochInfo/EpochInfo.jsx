import React from 'react';
import Table from 'react-bootstrap/Table';

import graphql from 'babel-plugin-relay/macro';
import { createFragmentContainer } from 'react-relay';

import { EmptyResult, BlockLink, CopiableItem, NextPrev, EpochDateTime } from '../../Commons';
import { getNextPrev } from '../../../helpers/epochHelper';

const EpochInfo = ({ epoch }) => {
  if (!epoch) {
    return <EmptyResult {...{ entityName: 'epoch' }} />;
  }
  const { firstBlock, lastBlock } = epoch;
  const baseUrl = '/epoch/';

  return (
    <div className="entityInfoTable">
      <NextPrev {...{ baseUrl, element: epoch, getNextPrev }} />
      <h2>Epoch</h2>
      <div className="keyValueTable">
        <Table striped bordered hover>
          <tbody>
            <tr>
              <td>Epoch Number:</td>
              <td>
                <CopiableItem text={epoch.id} />
              </td>
            </tr>
            <tr>
              <td>Date:</td>
              <td>
                <EpochDateTime {...{ epoch }} />
              </td>
            </tr>
            <tr>
              <td>First Block:</td>
              <td>{firstBlock && <BlockLink id={firstBlock.id} />}</td>
            </tr>
            <tr>
              <td>Last Block:</td>
              <td>{lastBlock && <BlockLink id={lastBlock.id} />}</td>
            </tr>
            <tr>
              <td>Blocks count:</td>
              <td>{epoch.totalBlocks}</td>
            </tr>
          </tbody>
        </Table>
      </div>
    </div>
  );
};

export default createFragmentContainer(EpochInfo, {
  epoch: graphql`
    fragment EpochInfo_epoch on Epoch {
      id
      firstBlock {
        id
      }
      lastBlock {
        id
      }
      totalBlocks
      ...EpochDateTime_epoch
    }
  `
});
