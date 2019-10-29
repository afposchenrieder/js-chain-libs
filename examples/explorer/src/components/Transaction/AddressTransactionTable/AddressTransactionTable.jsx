import React from 'react';

import graphql from 'babel-plugin-relay/macro';
import { createRefetchContainer } from 'react-relay';
import logger from '../../../logger/logger';

import TransactionTable from '../TransactionTable/TransactionTable';
import { pageNumberDesc, getDescPageQuery } from '../../../helpers/paginationHelper';

const AddressTransactionTable = ({ address, relay }) => {
  if (!address.transactions) {
    return null;
  }

  const connection = address.transactions;
  const currentPage = pageNumberDesc(connection);

  const handlePageChange = page => {
    const params = getDescPageQuery(page.current, connection.totalCount);
    const variables = {
      address: address.id,
      first: params.first || null,
      last: params.last || null,
      after: params.after || null,
      before: params.before || null
    };

    relay.refetch(variables, error => {
      if (error) {
        logger.error('There was an error refetching Address blocks');
      }
    });
  };

  return (
    <>
      <h2>Transactions</h2>
      <TransactionTable {...{ currentPage, connection, handlePageChange }} />
    </>
  );
};

export default createRefetchContainer(
  AddressTransactionTable,
  {
    address: graphql`
      fragment AddressTransactionTable_address on Address
        @argumentDefinitions(
          first: { type: "Int" }
          last: { type: "Int" }
          after: { type: "IndexCursor" }
          before: { type: "IndexCursor" }
        ) {
        id
        transactions(first: $first, last: $last, after: $after, before: $before) {
          edges {
            cursor
            node {
              id
              inputs {
                amount
              }
              outputs {
                amount
              }
              block {
                chainLength
              }
            }
          }
          pageInfo {
            hasNextPage
            hasPreviousPage
            endCursor
            startCursor
          }
          totalCount
        }
      }
    `
  },
  graphql`
    query AddressTransactionTableRefetchQuery(
      $address: String!
      $first: Int
      $last: Int
      $after: IndexCursor
      $before: IndexCursor
    ) {
      address(bech32: $address) {
        ...AddressTransactionTable_address
          @arguments(first: $first, last: $last, after: $after, before: $before)
      }
    }
  `
);
