import React from 'react';
import Table from 'react-bootstrap/Table';

import graphql from 'babel-plugin-relay/macro';
import { createFragmentContainer } from 'react-relay';

import { EmptyResult, CopiableItem, StakePoolLink, AddressLink } from '../../Commons';

const AddressInfo = ({ address }) => {
  if (!address) {
    return <EmptyResult {...{ entityName: 'address' }} />;
  }

  return (
    <div className="entityInfoTable">
      <h2>Address</h2>
      <div className="keyValueTable">
        <Table striped bordered hover responsive>
          <tbody>
            <tr>
              <td>Id:</td>
              <td>
                <CopiableItem text={address.id} />
              </td>
            </tr>
            <tr>
              <td>Delegation:</td>
              <td>{address.delegation.id && <StakePoolLink id={address.delegation.id} />}</td>
            </tr>
            <tr>
              <td>Type: </td>
              <td>{address.__typename && <CopiableItem text={address.__typename} />}</td>
            </tr>
            <tr>
              <td>KeyStake: </td>
              <td>{address.stakeKey && <AddressLink id={address.stakeKey.id} />}</td>
            </tr>
          </tbody>
        </Table>
      </div>
    </div>
  );
};

export default createFragmentContainer(AddressInfo, {
  address: graphql`
    fragment AddressInfo_address on Address {
      __typename
      id
      delegation {
        id
      }
      ... on UtxoAddress {
        stakeKey {
          id
        }
      }
    }
  `
});
